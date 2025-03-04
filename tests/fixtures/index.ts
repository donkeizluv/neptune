import * as utils from "../utils";
import { useConnection, useRoles } from "../setup/base";

// run once
export async function mochaGlobalSetup() {
  const { connection } = useConnection();
  const { player } = useRoles();
  await utils.massAirdrop([player.publicKey], connection, 111);
}
