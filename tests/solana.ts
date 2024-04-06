import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solana } from "../target/types/solana";

describe("solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solana as Program<Solana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();


    console.log("Your transaction signature", tx);
  });

  it("Test mint", async () => {


    // Add your test here.
    const tx = await program.methods.createDelegate().rpc();


    console.log("Your transaction signature", tx);
  });
});
