import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";
import { assert } from "chai";

describe("bvp_token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BvpToken as Program<BvpToken>;

  it("Initializes the token!", async () => {
    assert.ok(program.programId);
  });
});
