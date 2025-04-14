export type BvpToken = {
  "version": "0.1.0",
  "name": "bvp_token",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        { "name": "mint", "isMut": true, "isSigner": true },
        { "name": "user", "isMut": true, "isSigner": true },
        { "name": "systemProgram", "isMut": false, "isSigner": false },
        { "name": "rent", "isMut": false, "isSigner": false },
        { "name": "tokenProgram", "isMut": false, "isSigner": false }
      ],
      "args": [{ "name": "totalSupply", "type": "u64" }]
    },
    {
      "name": "stakeTokens",
      "accounts": [
        { "name": "user", "isMut": true, "isSigner": true },
        { "name": "tokenProgram", "isMut": false, "isSigner": false }
      ],
      "args": [
        { "name": "amount", "type": "u64" },
        { "name": "duration", "type": "u64" }
      ]
    }
  ],
  "accounts": [
    {
      "name": "Proposal",
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "id", "type": "u64" },
          { "name": "approvals", "type": "u8" },
          { "name": "rejections", "type": "u8" },
          { "name": "threshold", "type": "u8" },
          { "name": "executed", "type": "bool" }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Tier",
      "type": {
        "kind": "enum",
        "variants": [
          { "name": "None" },
          { "name": "Bronze" },
          { "name": "Silver" },
          { "name": "Gold" },
          { "name": "Platinum" },
          { "name": "Diamond" }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ProposalAlreadyExecuted",
      "msg": "This proposal has already been executed."
    }
  ],
  "metadata": {
    "address": "DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk"
  }
};
