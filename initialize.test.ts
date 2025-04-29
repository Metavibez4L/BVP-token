import * as anchor from "@coral-xyz/anchor";
import { BvpToken } from "../target/types/bvp_token";

// 👇 replace anchor.workspace.BvpToken
const programId = new anchor.web3.PublicKey("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");
const idl = require("../target/idl/bvp_token.json");

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = new anchor.Program(idl, programId, provider);
