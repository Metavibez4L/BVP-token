import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import { assert } from "chai";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("bvp_token - stakeTokens", function () {
  this.timeout(10000); // 10 seconds timeout

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BvpToken as Program<BvpToken>;
  const user = (provider.wallet as anchor.Wallet).payer;

  it("stakes tokens", async () => {
    console.log("Program ID:", program.programId.toBase58());
    console.log("User public key:", user.publicKey.toBase58());

    try {
      const tx = await program.methods
        .stakeTokens(new anchor.BN(100), new anchor.BN(30))
        .accounts({
          user: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();

      console.log("Stake transaction signature:", tx);
      assert.ok(tx);
    } catch (err) {
      console.error("❌ Error while staking tokens:", err);
      throw err;
    }
  });
});
