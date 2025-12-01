import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ParkingSim } from "../target/types/parking_sim";
import { assert } from "chai";

describe("parking_sim", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ParkingSim as Program<ParkingSim>;

  // Random ID to ensure a fresh test every time
  const spotId = `Spot_${Math.floor(Math.random() * 10000)}`;
  const licensePlate = "SOL-RICH";

  // Calculate the PDA
  const [spotPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("spot"), Buffer.from(spotId)],
    program.programId
  );

  it("1. Initializes the Spot (Assigns You as Manager)", async () => {
    await program.methods
      .initializeSpot(spotId)
      .accounts({
        spot: spotPda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verify the spot was created correctly
    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, false);
    assert.equal(spotAccount.lotOwner.toString(), provider.wallet.publicKey.toString());

    console.log(`✅ Spot ${spotId} Created`);
  });

  it("2. Parks Car & Pays 0.1 SOL to the Spot", async () => {
    // Check balance before
    const balanceBefore = await provider.connection.getBalance(spotPda);

    await program.methods
      .carPark(spotId, licensePlate)
      .accounts({
        spot: spotPda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Check balance after
    const balanceAfter = await provider.connection.getBalance(spotPda);
    const diff = balanceAfter - balanceBefore;

    // 0.1 SOL = 100,000,000 Lamports
    assert.equal(diff, 100_000_000);

    // Verify spot state changed
    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, true);
    assert.equal(spotAccount.licensePlate, licensePlate);
    assert.equal(spotAccount.carOwner.toString(), provider.wallet.publicKey.toString());
    // CRITICAL: Verify lot_owner didn't change!
    assert.equal(spotAccount.lotOwner.toString(), provider.wallet.publicKey.toString());

    console.log("✅ Car Parked. Spot received 0.1 SOL.");
  });

  it("3. Withdraws Profit (Manager pulls money out)", async () => {
    // 1. Snapshot Balances
    const spotBalanceBefore = await provider.connection.getBalance(spotPda);
    const userBalanceBefore = await provider.connection.getBalance(provider.wallet.publicKey);

    console.log(`💰 Spot Has: ${spotBalanceBefore / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    console.log(`💰 You Have: ${userBalanceBefore / anchor.web3.LAMPORTS_PER_SOL} SOL`);

    // 2. Action: Withdraw 0.05 SOL (Half the profit)
    const withdrawAmount = new anchor.BN(50_000_000);

    const tx = await program.methods
      .withdrawProfit(spotId, withdrawAmount)
      .accounts({
        spot: spotPda,
        signer: provider.wallet.publicKey, // You are the Manager
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`📝 Withdraw TX: ${tx}`);

    // 3. Verify Balances
    const spotBalanceAfter = await provider.connection.getBalance(spotPda);
    const userBalanceAfter = await provider.connection.getBalance(provider.wallet.publicKey);

    console.log(`💰 Spot Now: ${spotBalanceAfter / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    console.log(`💰 You Now:  ${userBalanceAfter / anchor.web3.LAMPORTS_PER_SOL} SOL`);

    // Assertion: Spot should have exactly 50,000,000 less
    const spotDiff = spotBalanceBefore - spotBalanceAfter;
    assert.equal(spotDiff, 50_000_000, "Spot should have 50M lamports less");

    // Get transaction fee to calculate exact user balance
    const txDetails = await provider.connection.getTransaction(tx, {
      maxSupportedTransactionVersion: 0
    });
    const txFee = txDetails?.meta?.fee || 5000; // Default to 5000 if we can't get fee

    // Assertion: User should have gained (withdraw - tx fee)
    const expectedUserIncrease = 50_000_000 - txFee;
    const actualUserIncrease = userBalanceAfter - userBalanceBefore;

    console.log(`📊 Expected increase: ${expectedUserIncrease}, Actual: ${actualUserIncrease}`);
    assert.approximately(actualUserIncrease, expectedUserIncrease, 1000,
      "User balance should increase by withdraw amount minus fees");

    console.log("✅ Profit Withdrawn Successfully");
  });

  it("4. Test: Cannot Withdraw Below Rent-Exempt Minimum", async () => {
    const spotBalance = await provider.connection.getBalance(spotPda);

    // Try to withdraw almost everything (should fail)
    const tooMuch = new anchor.BN(spotBalance - 1000); // Leave only 1000 lamports

    try {
      await program.methods
        .withdrawProfit(spotId, tooMuch)
        .accounts({
          spot: spotPda,
          signer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Should have thrown InsufficientBalance error");
    } catch (error) {
      assert.include(error.toString(), "InsufficientBalance");
      console.log("✅ Correctly prevented withdrawal below rent-exempt minimum");
    }
  });

  it("5. Test: Only Lot Owner Can Withdraw", async () => {
    // Create a different wallet
    const hacker = anchor.web3.Keypair.generate();

    // Airdrop some SOL to pay for transaction
    await provider.connection.requestAirdrop(
      hacker.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await new Promise(resolve => setTimeout(resolve, 1000)); // Wait for airdrop

    try {
      await program.methods
        .withdrawProfit(spotId, new anchor.BN(1000))
        .accounts({
          spot: spotPda,
          signer: hacker.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([hacker])
        .rpc();

      assert.fail("Should have thrown NotYourProfit error");
    } catch (error) {
      assert.include(error.toString(), "NotYourProfit");
      console.log("✅ Correctly prevented unauthorized withdrawal");
    }
  });
});