import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day02 } from "../target/types/day_02";

describe("day_02", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day02 as Program<Day02>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(777), new anchor.BN(888), "Hello World!")
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods
      .array([new anchor.BN(777), new anchor.BN(888)])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate add", async () => {
    const tx = await program.methods
      .add(new anchor.BN(5), new anchor.BN(3))
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate subtract", async () => {
    const tx = await program.methods
      .subtract(new anchor.BN(111), new anchor.BN(11))
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate multiply", async () => {
    const tx = await program.methods
      .multiply(new anchor.BN(2), new anchor.BN(3))
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate divide", async () => {
    const tx = await program.methods
      .divide(new anchor.BN(10), new anchor.BN(3))
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate sqrt", async () => {
    const tx = await program.methods
      .sqrt(new anchor.BN(10))
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("It should calculate log10", async () => {
    const tx = await program.methods
      .log10(new anchor.BN(55))
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
