import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const programId = new anchor.web3.PublicKey("9erFjuSDr2NwCUGmd1H1epfhMcy7aEF7g5xtTn71Eh3m");

(async () => {
  const idl = await anchor.Program.fetchIdl(programId, provider);
  
  if (!idl) {
    console.error("❌ IDL not found on devnet. Did you forget to run `anchor deploy`?");
    process.exit(1);
  }

  const program = new anchor.Program(idl, programId, provider);
  console.log("✅ Program loaded successfully:", program.programId.toBase58());
})();
