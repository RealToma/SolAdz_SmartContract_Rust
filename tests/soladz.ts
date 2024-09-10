import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Soladz } from "../target/types/soladz";
import { PublicKey } from "@solana/web3.js";

describe("soladz", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Soladz as Program<Soladz>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it ("ownership", async () => {
    const tx = await program.methods.transferOwnership().accounts({
      newOwner: new PublicKey('Gaj7cGbQ3CCWkqn8QsnLXEVaBaTN98GRxkX1pPsC4yNS')
    }).rpc();
    console.log(tx)
  })
});
