import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { BvpToken } from "../target/types/bvp_token";

describe("unstake", function() {
  // Allow longer for network round-trips
  this.timeout(15000);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new anchor.web3.PublicKey(
    "DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk"
  );
  const idl = require("../target/idl/bvp_token.json");
  const program = new anchor.Program(idl, programId, provider) as Program<BvpToken>;

  const user = provider.wallet.publicKey;

  it("stakes then unstakes, resetting amount to 0", async () => {
    const stakeStateKeypair = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000);
    const duration = new anchor.BN(3600);

    // 1) Stake (now requires both amount & duration)
    await program.methods
      .stakeTokens(amount, duration)
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakeStateKeypair])
      .rpc();

    // 2) Unstake (only stakeState & user are required)
    await program.methods
      .unstakeTokens()
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user,
      })
      .rpc();

    // 3) Verify the amount was zeroed out
    const stakeState = await program.account.stakeState.fetch(
      stakeStateKeypair.publicKey
    );
    assert.equal(stakeState.amount.toString(), "0");
  });
});
