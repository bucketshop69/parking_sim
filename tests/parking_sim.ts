import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ParkingSim } from "../target/types/parking_sim";
import { assert } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("parking_sim", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ParkingSim as Program<ParkingSim>;
  const wallet = provider.wallet as anchor.Wallet;

  it("initialize_lot", async () => {
    const [lotPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("parking_sim"), wallet.publicKey.toBuffer()],
      program.programId
    );

    const lotPdaAccount = await program.provider.connection.getAccountInfo(lotPda);

    if (!lotPdaAccount) {
      const tx = await program.methods.initializeLotAccount()
        .accounts({ signer: wallet.publicKey }).rpc()
      console.log("Transaction signature", tx);
    }
    const lotAccount = await program.account.lotAccount.fetch(lotPda);

    console.log("Owner:", lotAccount.owner.toBase58());
    console.log("Level:", lotAccount.level);
    console.log("Spots:", lotAccount.spots);

    assert.equal(lotAccount.level, 1);

  })


  it("initialize spots", async () => {
    const [lotPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("parking_sim"), wallet.publicKey.toBuffer()],
      program.programId
    );

    const lotPdaAccount = await program.provider.connection.getAccountInfo(lotPda);
    const botIndex = 1;
    const [botPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bot_account"), lotPda.toBuffer(), Buffer.from([botIndex])],
      program.programId
    )
    if (lotPdaAccount) {
      const tx = await program.methods.initializeBot(botIndex, lotPda)
        .accounts({ signer: wallet.publicKey, botAccount: botPDA })
        .rpc()
      console.log("Transaction signature", tx);


      if (botPDA) {

        const botAccount = await program.account.botAccount.fetch(botPDA);

        console.log("Bot index", botAccount.index);
        console.log("Bot Type", botAccount.botType);
        console.log("Lot", botAccount.lot.toString());

      }
    }
  })



});