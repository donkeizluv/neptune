import * as anchor from "@coral-xyz/anchor";
import { massAirdrop } from "./utils";
import { useConnection, useProgram, useRoles } from "./setup/base";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { deriveEscrow } from "./utils/wagmi";
import { LOCKED_VOTER_PROGRAM_ID } from "./utils/const";

describe("neptune", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = useProgram();
  const { connection } = useConnection();
  const { player } = useRoles();
  const jupLocker = new PublicKey(
    "CVMdMd79no569tjc5Sq7kzz8isbfCcFyBS5TLGsrZ5dN"
  );
  const [jupEscrow, _bump] = deriveEscrow(
    jupLocker,
    player.publicKey,
    LOCKED_VOTER_PROGRAM_ID
  );
  before(async () => {
    await massAirdrop([player.publicKey], connection, 12345);
  });
  it("Is initialized!", async () => {
    const accounts = {
      signer: player.publicKey,
      locker: jupLocker,
      escrow: jupEscrow,
      escrowOwner: player.publicKey,
      lockedVoter: LOCKED_VOTER_PROGRAM_ID,
      // systemProgram: SystemProgram.programId,
    };
    const tx = await program.methods
      .createVault()
      .accounts(accounts)
      .signers([player])
      .rpc({ skipPreflight: true });

    console.log("Your transaction signature", tx);
  });
});
