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
      newOwner: new PublicKey('q6j5TYe3pBcTZANEr1pfAGNhkh9sBvBiCQZi8zrP8Fm')
    }).rpc();
    console.log(tx)
  })
});
