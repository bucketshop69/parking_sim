import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ParkingSim } from "../target/types/parking_sim";
import { assert } from "chai";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("parking_sim", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ParkingSim as Program<ParkingSim>;
  const wallet = provider.wallet as anchor.Wallet;

  // Random ID to ensure a fresh test every time
  const spotId = `Spot_${Math.floor(Math.random() * 10000)}`;
  const licensePlate = "SOL-RICH";

  // Hourly rate: 0.5 tokens per hour (500_000 with 6 decimals)
  const hourlyRate = new anchor.BN(500_000);

  // PDAs
  const [spotPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("spot"), Buffer.from(spotId)],
    program.programId
  );

  const [parkMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("park_mint")],
    program.programId
  );

  // Token variables
  let paymentMint: anchor.web3.PublicKey;
  let userPaymentAta: anchor.web3.PublicKey;
  let spotPaymentAta: anchor.web3.PublicKey;
  let userParkAta: anchor.web3.PublicKey;

  // ============ SETUP ============

  it("0. Setup: Create a fake payment token and mint to user", async () => {
    paymentMint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    );
    console.log(`✅ Created payment mint: ${paymentMint.toBase58()}`);

    const userPaymentAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      paymentMint,
      wallet.publicKey
    );
    userPaymentAta = userPaymentAccount.address;

    await mintTo(
      provider.connection,
      wallet.payer,
      paymentMint,
      userPaymentAta,
      wallet.publicKey,
      100_000_000
    );

    const balance = await getAccount(provider.connection, userPaymentAta);
    console.log(`✅ User has ${Number(balance.amount) / 1_000_000} payment tokens`);
  });

  it("1. Initializes the PARK token mint", async () => {
    const mintInfo = await provider.connection.getAccountInfo(parkMintPda);
    if (mintInfo) {
      console.log(`ℹ️ PARK mint already exists at: ${parkMintPda.toBase58()}`);
      return;
    }

    await program.methods
      .initializeParkMint()
      .accounts({
        parkMint: parkMintPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log(`✅ PARK mint created at: ${parkMintPda.toBase58()}`);
  });

  it("2. Initializes the Spot with hourly rate", async () => {
    await program.methods
      .initializeSpot(spotId, hourlyRate)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, false);
    assert.equal(spotAccount.hourlyRate.toNumber(), hourlyRate.toNumber());

    console.log(`✅ Spot ${spotId} created with rate: ${hourlyRate.toNumber() / 1_000_000} tokens/hour`);
  });

  it("3. Parks Car (no payment yet - just records time)", async () => {
    await program.methods
      .carPark(spotId, licensePlate)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,  // Add this
      })
      .rpc();
    // ...
  });

  it("4. Leave Parking: Pays based on time + receives PARK reward", async () => {
    // Calculate ATAs
    spotPaymentAta = anchor.utils.token.associatedAddress({
      mint: paymentMint,
      owner: spotPda,
    });

    userParkAta = anchor.utils.token.associatedAddress({
      mint: parkMintPda,
      owner: wallet.publicKey,
    });

    // Get balances before
    const paymentBefore = await getAccount(provider.connection, userPaymentAta);
    console.log(`💰 User payment tokens before: ${Number(paymentBefore.amount) / 1_000_000}`);

    let parkBefore = BigInt(0);
    try {
      const parkAccountBefore = await getAccount(provider.connection, userParkAta);
      parkBefore = parkAccountBefore.amount;
      console.log(`💰 User PARK tokens before: ${Number(parkBefore) / 1_000_000_000}`);
    } catch {
      console.log(`💰 User has no PARK tokens yet`);
    }

    // Leave parking (this triggers payment)
    await program.methods
      .leaveParking(spotId)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        paymentMint: paymentMint,
        userPaymentAta: userPaymentAta,
        spotPaymentAta: spotPaymentAta,
        parkMint: parkMintPda,
        userParkAta: userParkAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verify payment (minimum 1 hour = 0.5 tokens = 500_000)
    const paymentAfter = await getAccount(provider.connection, userPaymentAta);
    const paymentDiff = Number(paymentBefore.amount) - Number(paymentAfter.amount);
    assert.equal(paymentDiff, 500_000, "Should have paid 0.5 tokens (1 hour minimum)");
    console.log(`✅ Paid ${paymentDiff / 1_000_000} tokens for parking`);

    // Verify spot received payment
    const spotPayment = await getAccount(provider.connection, spotPaymentAta);
    console.log(`✅ Spot received ${Number(spotPayment.amount) / 1_000_000} tokens`);

    // Verify PARK reward (1 PARK per hour)
    const userPark = await getAccount(provider.connection, userParkAta);
    const parkDiff = Number(userPark.amount) - Number(parkBefore);
    assert.equal(parkDiff, 1_000_000_000, "Should have received 1 PARK (1 hour)");
    console.log(`✅ Received ${parkDiff / 1_000_000_000} PARK tokens as reward`);

    // Verify spot is cleared
    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, false);
    assert.equal(spotAccount.parkedAt.toNumber(), 0);

    console.log("✅ Left parking successfully!");
  });

  it("5. Owner withdraws profit", async () => {
    const spotPaymentBefore = await getAccount(provider.connection, spotPaymentAta);
    console.log(`💰 Spot has: ${Number(spotPaymentBefore.amount) / 1_000_000} tokens`);

    const ownerPaymentAta = anchor.utils.token.associatedAddress({
      mint: paymentMint,
      owner: wallet.publicKey,
    });

    await program.methods
      .withdrawProfit(spotId, new anchor.BN(Number(spotPaymentBefore.amount)))
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        paymentMint: paymentMint,
        spotPaymentAta: spotPaymentAta,
        ownerPaymentAta: ownerPaymentAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const spotPaymentAfter = await getAccount(provider.connection, spotPaymentAta);
    assert.equal(Number(spotPaymentAfter.amount), 0, "Spot should be empty");

    console.log(`✅ Owner withdrew all tokens`);
  });
});