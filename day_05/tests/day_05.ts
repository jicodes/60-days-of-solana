import * as anchor from "@coral-xyz/anchor";

import fs from "fs";
let idl = JSON.parse(fs.readFileSync("target/idl/day_05.json", "utf-8"));

describe("deployed", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Change this to be your programID
  const program = new anchor.Program(idl, provider);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
