import { web3 } from "@coral-xyz/anchor";
import { SolDecimals, toNativeAmount } from "@packages/common";
import {
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

export const commitment: web3.Commitment = "processed";

export async function airdrop(
  pubKey: web3.PublicKey,
  connection: web3.Connection,
  amount = 100
) {
  const signature = await connection.requestAirdrop(
    pubKey,
    toNativeAmount(amount, SolDecimals)
  );
  const latestBlockhash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
}
export async function massAirdrop(
  keys: Array<web3.PublicKey>,
  connection: web3.Connection,
  amount = 100
) {
  for (const key of keys) {
    await airdrop(key, connection, amount);
  }
}

export const paramsToArray = (p) => Object.keys(p).map((v) => p[v]);

export const getCurrentUnixTimestamp = () => Math.floor(Date.now() / 1000);

export async function getOrCreateATA(
  mint: web3.PublicKey,
  owner: web3.PublicKey,
  payer: web3.Keypair,
  connection: web3.Connection
) {
  const ata = getAssociatedTokenAddressSync(mint, owner, true);

  const account = await connection.getAccountInfo(ata);

  if (!account) {
    const tx = await web3.sendAndConfirmTransaction(
      connection,
      new web3.Transaction().add(
        createAssociatedTokenAccountInstruction(
          payer.publicKey,
          ata,
          owner,
          mint
        )
      ),
      [payer]
    );

    await connection.confirmTransaction(tx, "confirmed");
  }

  return ata;
}

export async function getOnChainTime(
  connection: web3.Connection
): Promise<number> {
  const parsedClock = await connection.getParsedAccountInfo(
    web3.SYSVAR_CLOCK_PUBKEY
  );

  const parsedClockAccount = (parsedClock.value!.data as web3.ParsedAccountData)
    .parsed as any;

  const currentTime = parsedClockAccount.info.unixTimestamp;
  return currentTime as number;
}
