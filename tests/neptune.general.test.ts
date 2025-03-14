import * as anchor from "@coral-xyz/anchor";
import { Neptune } from "../target/types/neptune";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("neptune", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Neptune as Program<Neptune>;

  before(async () => {});
  it("Is initialized!", async () => {
    const blockhash = await provider.connection.getLatestBlockhash();
  });
});
