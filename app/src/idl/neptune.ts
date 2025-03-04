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
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
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
  ]
};
