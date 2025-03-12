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
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "escrow"
              }
            ]
          }
        },
        {
          "name": "lstMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  115,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "vault"
              }
            ]
          }
        },
        {
          "name": "utokenMint"
        },
        {
          "name": "locker",
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  69,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "locker"
              },
              {
                "kind": "account",
                "path": "vault"
              }
            ]
          }
        },
        {
          "name": "vaultOwner"
        },
        {
          "name": "lockedVoter",
          "address": "voTpe3tHQ7AjQHMapgSue2HJFAh2cGsdokqN3XqmVSj"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "feesBps",
          "type": "u16"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "locker",
      "discriminator": [
        74,
        246,
        6,
        113,
        249,
        228,
        75,
        169
      ]
    },
    {
      "name": "vault",
      "discriminator": [
        211,
        8,
        232,
        43,
        2,
        152,
        117,
        119
      ]
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
      "name": "invalidUnstakeAmt"
    },
    {
      "code": 6004,
      "name": "amtMustGreaterThanZero"
    },
    {
      "code": 6005,
      "name": "invalidBps"
    },
    {
      "code": 6006,
      "name": "escrowAmtIsNotCorrect"
    }
  ],
  "types": [
    {
      "name": "locker",
      "docs": [
        "A group of [Escrow]s."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "base",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "lockedSupply",
            "type": "u64"
          },
          {
            "name": "totalEscrow",
            "type": "u64"
          },
          {
            "name": "governor",
            "type": "pubkey"
          },
          {
            "name": "params",
            "type": {
              "defined": {
                "name": "lockerParams"
              }
            }
          },
          {
            "name": "buffers",
            "type": {
              "array": [
                "u128",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "lockerParams",
      "docs": [
        "Contains parameters for the [Locker]."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxStakeVoteMultiplier",
            "type": "u8"
          },
          {
            "name": "minStakeDuration",
            "type": "u64"
          },
          {
            "name": "maxStakeDuration",
            "type": "u64"
          },
          {
            "name": "proposalActivationMinVotes",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "vault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "escrow",
            "type": "pubkey"
          },
          {
            "name": "lstMint",
            "type": "pubkey"
          },
          {
            "name": "totalLstMinted",
            "type": "u64"
          },
          {
            "name": "totalUtokenStaked",
            "type": "u64"
          },
          {
            "name": "feesBps",
            "type": "u16"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
