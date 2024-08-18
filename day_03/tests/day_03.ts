import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day03 } from "../target/types/day_03";

describe("day_03", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day03 as Program<Day03>;

  it("Should add", async () => {
    const tx = await program.methods
      .add(new anchor.BN(1), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should subtract", async () => {
    const tx = await program.methods
      .sub(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should multiply", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(5), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should divide", async () => {
    const tx = await program.methods
      .div(new anchor.BN(10), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should modulo", async () => {
    const tx = await program.methods
      .modulo(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
