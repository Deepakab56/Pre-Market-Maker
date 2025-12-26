import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PreMarket } from "../target/types/pre_market";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createAccount,
  createMint,
  getAssociatedTokenAddress,
  mintTo,
  ONE_IN_BASIS_POINTS,
} from "@solana/spl-token";
import BN from "bn.js";
import { assert } from "chai";
describe("pre-market", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.preMarket as Program<PreMarket>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  let mintUSDC: PublicKey;
  let userTokenAccount: PublicKey;
  let tokenDetailsPda: PublicKey;
  let orderPda: PublicKey;
  let marketIdPda: PublicKey;

  const CREATE_ID = new BN(0);

  before(async () => {
    // 1️⃣ Create USDC mint
    mintUSDC = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      9
    );

    // 2️⃣ Create user token account
    userTokenAccount = await createAccount(
      connection,
      wallet.payer,
      mintUSDC,
      wallet.publicKey
    );

    // 3️⃣ Mint tokens
    await mintTo(
      connection,
      wallet.payer,
      mintUSDC,
      userTokenAccount,
      wallet.publicKey,
      100_000 * LAMPORTS_PER_SOL
    );
  });

  it("Fetch MarketIds PDA", async () => {
    // const tx = await program.methods.initialize().rpc();
    // console.log(tx);
    [marketIdPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("create_id")],
      program.programId
    );

    const marketIds = await program.account.marketIds.fetch(marketIdPda);
    assert.ok(marketIds);
  });

  it("Initialize Token Details", async () => {
    await program.methods.initTokenDetails("AB", "AB").rpc();

    const buffer = CREATE_ID.toArrayLike(Buffer, "le", 8);

    [tokenDetailsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_details"), buffer],
      program.programId
    );

    // const key= new PublicKey("HowFz4NTtgZ95rr6N9HuZc2vXqb51Tvr5nHbgfvNMeJu");
    console.log({ tokenDetailsPda });
    const tokenDetails = await program.account.tokenDetails.fetch(
      tokenDetailsPda
    );
    console.log({ tokenDetails });
    // assert.equal(tokenDetails.creator.toBase58(), wallet.publicKey.toBase58());
  });

  it("Create Order (SELL)", async () => {
    const marketIds = await program.account.marketIds.fetch(marketIdPda);
    const orderId = marketIds.orderId;

    const [orderPdaAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("create_order"),
        new BN(0).toArrayLike(Buffer, "le", 8),
        new BN(0).toArrayLike(Buffer, "be", 8),
      ],
      program.programId
    );

    // console.log({ orderPdaAddress });

    orderPda = orderPdaAddress;

    const accountsecond = await getAssociatedTokenAddress(
      mintUSDC,
      orderPda,
      true
    );

    await program.methods
      .initOrder(
        new BN(1_000), // pointAmount
        new BN(1_000), // pointPrice
        true, // isPartial
        CREATE_ID
      )
      .accounts({
        signer: wallet.publicKey,
        mint: mintUSDC,
        idAccount: marketIdPda,
        tokenDetailsAccount: tokenDetailsPda,
        orderAccountDetails: orderPda,
        userTokenAccount: userTokenAccount,
        detailsTokenAccount: accountsecond,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const order = await program.account.orderBook.fetch(orderPda);

    assert.equal(order.orderCreator.toBase58(), wallet.publicKey.toBase58());
    assert.equal(order.offerType.sell !== undefined, true);
    assert.equal(order.isPartial, true);
  });
});
