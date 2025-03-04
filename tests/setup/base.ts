import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import * as dotenv from "dotenv";
import { Neptune } from "../../target/types/neptune";

dotenv.config();

export const commitment: web3.Commitment = "processed";
const RPC_CONNECTION = "http://localhost:8899";

export function useConnection() {
  return {
    connection: new web3.Connection(RPC_CONNECTION, {
      commitment,
    }),
  };
}

export function useProgram() {
  return anchor.workspace.Neptune as Program<Neptune>;
}

const player = web3.Keypair.generate();

export function useRoles() {
  return {
    player,
  };
}
