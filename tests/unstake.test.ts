import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import { assert } from "chai";

describe("unstake", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new anchor.web3.PublicKey("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");
  const idl = require("../target/idl/bvp_token.json");
  const program = new anchor.Program(idl, programId, provider) as Program<BvpToken>;

  it("stakes then unstakes, resetting amount to 0", async () => {
    // 1) Stake
    const stakeStateKeypair = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000);
    const duration = new anchor.BN(3600);

    await program.methods
      .stakeTokens(amount, duration)
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([stakeStateKeypair])
      .rpc();

    // Verify stake applied
    let state = await program.account.stakeState.fetch(stakeStateKeypair.publicKey);
    assert.equal(state.amount.toString(), amount.toString());

    // 2) Unstake
    await program.methods
      .unstakeTokens()
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Verify amount reset
    state = await program.account.stakeState.fetch(stakeStateKeypair.publicKey);
    assert.equal(state.amount.toString(), "0");
  });
});
