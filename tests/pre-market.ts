import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PreMarket } from "../target/types/pre_market";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { PublicKey } from "@solana/web3.js";

describe("pre-market", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.preMarket as Program<PreMarket>;

  // console.log(program);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log(tx);

    const [idPool] = PublicKey.findProgramAddressSync(
      [Buffer.from("create_id")],
      program.programId
    );
    // console.log(idPool.toBase58());
    const data = await program.account.marketIds.fetch(idPool);
    console.log(data);
  });
  it("if user create trade", async () => {
    const tx = await program.methods.initTokenDetails("AB", "AB").rpc();
    console.log(tx)
  });
});
