import { AnchorProvider, utils } from "@coral-xyz/anchor";
import {
  AddressLookupTableAccount,
  BlockhashWithExpiryBlockHeight,
  ConfirmOptions,
  Connection,
  PublicKey,
  SendTransactionError,
  Signer,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  TransactionSignature,
  VersionedTransaction,
} from "@solana/web3.js";

/**
 * Sends the given transaction, paid for and signed by the provider's wallet.
 *
 * @param provider     The Anchor provider
 * @param instructions The transaction to send.
 * @param signers      The signers of the transaction.
 * @param opts         Transaction confirmation options.
 */
export async function sendAndConfirm(
  provider: AnchorProvider,
  instructions: TransactionInstruction[],
  opts?: ConfirmOptions,
  signers?: Signer[]
): Promise<TransactionSignature> {
  if (opts === undefined) {
    opts = provider.opts;
  }

  const { value: blockhash } =
    await provider.connection.getLatestBlockhashAndContext(
      opts.preflightCommitment
    );

  const transaction = new Transaction({
    feePayer: provider.wallet.publicKey,
    blockhash: blockhash.blockhash,
    lastValidBlockHeight: blockhash.lastValidBlockHeight,
  }).add(...instructions);

  if (signers?.length) {
    transaction.partialSign(...signers);
  }
  const signedTransaction = await provider.wallet.signTransaction(transaction);

  const rawTx = signedTransaction.serialize();

  try {
    return await sendAndConfirmRawTransaction(
      provider.connection,
      rawTx,
      blockhash,
      opts
    );
  } catch (err: any) {
    // thrown if the underlying 'confirmTransaction' encounters a failed tx
    // the 'confirmTransaction' error does not return logs so we make another rpc call to get them
    // choose the shortest available commitment for 'getTransaction'
    // (the json RPC does not support any shorter than "confirmed" for 'getTransaction')
    // because that will see the tx sent with `sendAndConfirmRawTransaction` no matter which
    // commitment `sendAndConfirmRawTransaction` used

    // no need to confirm sim failed since it hasnt been sent
    if (
      err instanceof SendTransactionError &&
      err.message.includes("Transaction simulation failed")
    ) {
      throw err;
    }

    // confirm & fetch fail message
    await provider.connection.confirmTransaction(
      {
        ...blockhash,
        signature: utils.bytes.bs58.encode(signedTransaction.signature!),
      },
      opts && opts.commitment
    );

    const failedTx = await provider.connection.getTransaction(
      utils.bytes.bs58.encode(signedTransaction.signature!),
      {
        commitment: "confirmed",
        maxSupportedTransactionVersion: 0,
      }
    );

    const logs = failedTx?.meta?.logMessages;
    const message = `${err.message}\n${JSON.stringify(logs, undefined, 2)}`;

    throw !logs ? err : new SendTransactionError(message);
  }
}

/**
 * Sends the given transaction, paid for and signed by the provider's wallet.
 *
 * @param provider     The Anchor provider
 * @param instructions The transaction to send.
 * @param signers      The signers of the transaction.
 * @param opts         Transaction confirmation options.
 */
export async function sendAndConfirmV0(
  provider: AnchorProvider,
  instructions: TransactionInstruction[][],
  lookupTables: LookupTable[],
  signers?: Signer[],
  opts?: ConfirmOptions
): Promise<TransactionSignature> {
  if (opts === undefined) {
    opts = provider.opts;
  }

  const tables: AddressLookupTableAccount[] = [];
  for (const table of lookupTables) {
    const lookupTable = new AddressLookupTableAccount({
      key: new PublicKey(table.address),
      state: AddressLookupTableAccount.deserialize(table.data),
    });
    tables.push(lookupTable);
  }

  const { value: blockhash } =
    await provider.connection.getLatestBlockhashAndContext(
      opts.preflightCommitment
    );

  const transactions = instructions.map((ix) => {
    const message = new TransactionMessage({
      payerKey: provider.wallet.publicKey,
      recentBlockhash: blockhash.blockhash,
      instructions: ix,
    }).compileToV0Message(tables);
    const transaction = new VersionedTransaction(message);
    if (signers?.length) {
      transaction.sign(signers);
    }
    return transaction;
  });

  // This works, but ideally we shouldn't have to cast the v0tx
  const signedTransaction = await provider.wallet.signAllTransactions(
    transactions as any
  );

  let lastTxn = "";
  try {
    for (let i = 0; i < signedTransaction.length; i++) {
      const transaction = signedTransaction[i];
      const rawTx = transaction.serialize();
      const sent = await sendAndConfirmRawTransaction(
        provider.connection,
        Buffer.from(rawTx),
        {
          ...blockhash,
        },
        opts
      ).catch((err) => {
        let customErr = toConfirmError(err.message);
        console.log(customErr, transaction);
        throw customErr;
      });
      lastTxn = sent;
    }
  } catch (e: any) {
    throw e;
  }
  return lastTxn;
}

/**
 * Sends all transactions. If an entry in the transactions array is
 * a sub-array, then transactions within the sub array are sent in parallel
 */
export async function sendAll(
  provider: AnchorProvider,
  transactions: (TransactionInstruction[] | TransactionInstruction[][])[],
  opts?: ConfirmOptions
): Promise<string> {
  if (opts === undefined) {
    opts = provider.opts;
  }

  let blockhash = (
    await provider.connection.getLatestBlockhashAndContext(
      opts.preflightCommitment
    )
  ).value;

  const txs = transactions
    .map((tx) => {
      if (Array.isArray(tx[0])) {
        return tx
          .map((tx: any) => {
            const ixs = tx as any as TransactionInstruction[];
            if (ixs.length > 0) {
              return new Transaction({
                feePayer: provider.wallet.publicKey,
                blockhash: blockhash.blockhash,
                lastValidBlockHeight: blockhash.lastValidBlockHeight,
              }).add(...ixs);
            }
            return;
          })
          .filter((tx) => !!tx) as Transaction[];
      } else {
        const ixs = tx as any as TransactionInstruction[];
        if (ixs.length > 0) {
          return [
            new Transaction({
              feePayer: provider.wallet.publicKey,
              blockhash: blockhash.blockhash,
              lastValidBlockHeight: blockhash.lastValidBlockHeight,
            }).add(...ixs),
          ];
        }
      }
      return;
    })
    .filter((tx) => !!tx) as (Transaction | Transaction[])[];

  let start = 0;
  const slices = txs.map((tx) => {
    const length = Array.isArray(tx) ? tx.length : 1;
    const end = start + length;
    const slice = [start, end];
    start = end;
    return slice;
  });

  // signedTxs has been flattened. unflatten it
  const signedTxs = await provider.wallet.signAllTransactions(txs.flat(1));
  const signedUnflattened = slices.map((slice) => signedTxs.slice(...slice));

  let lastTxn = "";
  try {
    for (let i = 0; i < signedUnflattened.length; i++) {
      const transactions = signedUnflattened[i];
      const txnArray: string[] = [];
      for (const tx of transactions) {
        const rawTx = tx.serialize();
        const sent = await sendAndConfirmRawTransaction(
          provider.connection,
          rawTx,
          blockhash,
          opts
        ).catch((err) => {
          let customErr = toConfirmError(err.message);
          customErr.signature = utils.bytes.bs58.encode(tx.signature!);
          throw customErr;
        });
        txnArray.push(sent);
      }
      // Return the txid of the final transaction in the array
      // TODO: We should return an array instead of only the final txn
      lastTxn = txnArray[txnArray.length - 1] ?? "";
    }
  } catch (e: any) {
    throw e;
  }
  return lastTxn;
}

// Copy of Connection.sendAndConfirmRawTransaction that throws
// a better error if 'confirmTransaction` returns an error status
async function sendAndConfirmRawTransaction(
  connection: Connection,
  rawTransaction: Buffer,
  blockhashContext: BlockhashWithExpiryBlockHeight,
  options?: ConfirmOptions
): Promise<TransactionSignature> {
  const sendOptions = options && {
    skipPreflight: options.skipPreflight,
    preflightCommitment: options.preflightCommitment || options.commitment,
  };

  const signature = await connection.sendRawTransaction(
    rawTransaction,
    sendOptions
  );

  const status = (
    await connection.confirmTransaction(
      {
        signature,
        ...blockhashContext,
      },
      options && options.commitment
    )
  ).value;

  if (status.err) {
    const error = toConfirmError(
      `Raw transaction ${signature} failed (${JSON.stringify(status)})`
    );
    throw error;
  }

  return signature;
}

type ConfirmError = Error & {
  signature?: string;
};
const toConfirmError = (message: string, signature?: string): ConfirmError => ({
  ...new Error(message),
  signature,
});

type LookupTable = {
  address: string;
  // Avoid storing a class, and having to import the lookup table types
  data: Uint8Array;
};
