import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

export const assertAnchorError = async (
  rpc: () => Promise<any>,
  errorCode: string,
  message: string,
  errorCallback: (e: AnchorError) => void = null
) => {
  try {
    await rpc();
    assert.fail(message);
  } catch (error) {
    if (errorCallback) errorCallback(error);

    assert.ok(error instanceof AnchorError);
    const err: AnchorError = error;
    const actualErrorCode = err?.error?.errorCode?.code;
    assert.strictEqual(actualErrorCode, errorCode, message);
  }
};
