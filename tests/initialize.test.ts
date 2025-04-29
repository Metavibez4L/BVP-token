import * as anchor from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";

const programId = new anchor.web3.PublicKey("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");
const idl = require("../target/idl/bvp_token.json");

describe("initialize program manually", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(idl, programId, provider) as anchor.Program<BvpToken>;

  it("confirms the program ID is set", async () => {
    console.log("Program ID:", program.programId.toBase58());
  });
});
