/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/neptune.json`.
 */
export type Neptune = {
  "address": "DxQiCxj7hPw5oCXt4uMxXrsp1CLBmRUXzZczUwH9C5VU",
  "metadata": {
    "name": "neptune",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createVault",
      "discriminator": [
        29,
        237,
        247,
        208,
        193,
        82,
        54,
        135
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "locker",
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true
        },
        {
          "name": "escrowOwner"
        },
        {
          "name": "lockedVoter"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "You do not have sufficient permissions to perform this action."
    },
    {
      "code": 6001,
      "name": "cannotGetBump",
      "msg": "Cannot get the bump."
    },
    {
      "code": 6002,
      "name": "arithmeticOverflow"
    },
    {
      "code": 6003,
      "name": "invalidRedeemAmt"
    }
  ]
};
