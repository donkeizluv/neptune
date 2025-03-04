// helper functions copied from WAGMI repo
import { web3 } from "@coral-xyz/anchor";

export function deriveLocker(
  basePubkey: web3.PublicKey,
  programId: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("Locker"), basePubkey.toBytes()],
    programId
  );
}

export function deriveEscrow(
  locker: web3.PublicKey,
  escrowOwner: web3.PublicKey,
  voterProgram: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("Escrow"), locker.toBytes(), escrowOwner.toBytes()],
    voterProgram
  );
}
