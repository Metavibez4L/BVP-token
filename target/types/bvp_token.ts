export type BvpToken = {
  "version": "0.1.0",
  "name": "bvp_token",
  "instructions": [
    {
      "name": "stakeTokens",
      "accounts": [
        {
          "name": "stakeState",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "duration",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "stakeState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "duration",
            "type": "u64"
          }
        ]
      }
    }
  ]
};

export const IDL: BvpToken = {
  "version": "0.1.0",
  "name": "bvp_token",
  "instructions": [
    {
      "name": "stakeTokens",
      "accounts": [
        {
          "name": "stakeState",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "duration",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "stakeState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "duration",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
