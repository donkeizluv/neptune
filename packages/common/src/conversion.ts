import { SolDecimals } from "./consts";

// TODO handle big numbers
export function toDisplayAmount(amount: number, decimals: number): number {
  return amount / 10 ** (decimals || SolDecimals);
}

export function toNativeAmount(amount: number, decimals: number): number {
  return amount * 10 ** decimals;
}
