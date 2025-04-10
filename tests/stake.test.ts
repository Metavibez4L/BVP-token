import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import * as spl from "@solana/spl-token";
import { assert } from "chai";

describe("bvp_token - stakeTokens", function () {
  this.timeout(10000); // ⏱️ Increase timeout to 10 seconds

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BvpToken as Program<BvpToken>;
  console.log("Program ID:", program.programId.toBase58());

  it("stakes tokens", async () => {
    const user = anchor.web3.Keypair.generate();
    const tokenMint = anchor.web3.Keypair.generate().publicKey;

    // Airdrop to make sure user exists and has SOL
    await provider.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await new Promise(resolve => setTimeout(resolve, 2000)); // wait for airdrop confirmation

    const amount = 1000;
    const duration = 30;

    await program.methods
      .stakeTokens(new anchor.BN(amount), new anchor.BN(duration))
      .accounts({
        tokenMint,
        user: user.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    assert.ok(true);
  });
});
