import {
  TransactionInstruction,
  Connection,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  NATIVE_MINT,
  createSyncNativeInstruction,
  createCloseAccountInstruction,
} from "@solana/spl-token";
import { BN } from "@coral-xyz/anchor";

async function withCreateAta(
  instructions: TransactionInstruction[],
  connection: Connection,
  owner: PublicKey,
  mint: PublicKey,
): Promise<PublicKey> {
  const ata = getAssociatedTokenAddressSync(mint, owner);
  const account = await connection.getAccountInfo(ata);

  if (!account) {
    const ix = createAssociatedTokenAccountInstruction(owner, ata, owner, mint);
    instructions.push(ix);
  }
  return ata;
}

export async function addWrapIfNative(
  connection: Connection,
  owner: PublicKey,
  mint: PublicKey,
  amount: BN,
  ixs: TransactionInstruction[] = [],
): Promise<Array<TransactionInstruction>> {
  if (!mint.equals(NATIVE_MINT)) {
    return ixs;
  }

  const ata = await withCreateAta(ixs, connection, owner, mint);
  const transferIx = SystemProgram.transfer({
    fromPubkey: owner,
    lamports: amount.toNumber(),
    toPubkey: ata,
  });
  ixs.push(transferIx, createSyncNativeInstruction(ata));

  return ixs;
}

export async function addUnwrapIfNative(
  owner: PublicKey,
  mint: PublicKey,
  ixs: TransactionInstruction[] = [],
): Promise<Array<TransactionInstruction>> {
  if (!mint.equals(NATIVE_MINT)) {
    return ixs;
  }

  const tokenAddress = getAssociatedTokenAddressSync(mint, owner);
  ixs.push(createCloseAccountInstruction(tokenAddress, owner, owner));

  return ixs;
}
