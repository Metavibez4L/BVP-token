import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import { assert } from "chai";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token"; // ✅ Correct import

describe("stake-state", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new anchor.web3.PublicKey("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");
  const idl = require("../target/idl/bvp_token.json");
  const program = new anchor.Program(idl, programId, provider) as Program<BvpToken>;

  const user = provider.wallet;

  it("Stakes tokens and initializes stake state", async () => {
    const stakeStateKeypair = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000);
    const duration = new anchor.BN(3600); // 1 hour

    await program.methods
      .stakeTokens(amount, duration)
      .accounts({
        stakeState: stakeStateKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID, // ✅ Hardcoded from real SPL Token program address
      })
      .signers([stakeStateKeypair])
      .rpc();

    const stakeState = await program.account.stakeState.fetch(stakeStateKeypair.publicKey);

    assert.ok(stakeState.user.equals(user.publicKey));
    assert.equal(stakeState.amount.toString(), amount.toString());
    assert.equal(stakeState.duration.toString(), duration.toString());
    assert.ok(stakeState.startTimestamp.toNumber() > 0);
  });
});
