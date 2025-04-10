export type BvpToken = {
  version: "0.1.0";
  name: "bvp_token";
  instructions: [
    {
      name: "initialize";
      accounts: [
        { name: "tokenMint"; isMut: true; isSigner: false },
        { name: "user"; isMut: true; isSigner: true },
        { name: "systemProgram"; isMut: false; isSigner: false }
      ];
      args: [
        { name: "totalSupply"; type: "u64" }
      ];
    },
    {
      name: "stakeTokens";
      accounts: [
        { name: "tokenMint"; isMut: false; isSigner: false },
        { name: "user"; isMut: true; isSigner: true },
        { name: "tokenProgram"; isMut: false; isSigner: false }
      ];
      args: [
        { name: "amount"; type: "u64" },
        { name: "duration"; type: "u64" }
      ];
    }
  ];
};
