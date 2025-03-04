import { Connection, PublicKey } from "@solana/web3.js";
import * as spl from "@solana/spl-token";

export function cleanControlChars(str: string) {
  return str.replace(/[\u0000-\u001F\u007F-\u009F]/g, "");
}

export function shuffle(arr: Array<any>) {
  var j, x, index;
  for (index = arr.length - 1; index > 0; index--) {
    j = Math.floor(Math.random() * (index + 1));
    x = arr[index];
    arr[index] = arr[j];
    arr[j] = x;
  }
  return arr;
}

export async function getNFTOwner(
  connection: Connection,
  mint: PublicKey
): Promise<PublicKey | null> {
  const holders = await connection.getTokenLargestAccounts(mint);

  if (!holders || holders.value.length < 1) return null;
  const nftOwner = holders.value[0].address;

  const ata = await spl.getAccount(connection, nftOwner);

  if (!ata || ata.amount.toString() === "0") return null;
  if (ata.amount > 1) {
    throw new Error("mint is not NFT");
  }

  return ata.owner;
}

export const getUnixTimestamp = () => Math.floor(Date.now() / 1000);

export const clearControlChars = (val: string) =>
  val.replace(/[\u0000-\u001F\u007F-\u009F]/g, "");
