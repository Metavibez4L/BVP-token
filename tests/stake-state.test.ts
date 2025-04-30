import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import { assert } from "chai";

describe("stake-state", function() {
  // extend timeout to allow network round trips
  this.timeout(10000);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new anchor.web3.PublicKey(
    "DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk"
  );
  const idl = require("../target/idl/bvp_token.json");
  const program = new anchor.Program(idl, programId, provider) as Program<BvpToken>;

  const user = provider.wallet.publicKey;

  it("Stakes tokens and initializes stake state", async () => {
    const stakeStateKeypair = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000);
    const duration = new anchor.BN(3600); // 1 hour

    await program.methods
      .stakeTokens(amount, duration)
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakeStateKeypair])
      .rpc();

    const stakeState = await program.account.stakeState.fetch(
      stakeStateKeypair.publicKey
    );

    assert.ok(stakeState.user.equals(user));
    assert.equal(stakeState.amount.toString(), amount.toString());
    assert.equal(stakeState.duration.toString(), duration.toString());
    // the field name in the IDL is `startTimestamp`
    assert.ok(stakeState.startTimestamp.toNumber() > 0);
  });
});
