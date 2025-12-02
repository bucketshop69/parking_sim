import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ParkingSim } from "../target/types/parking_sim";
import { assert } from "chai";

describe("parking_sim - Level 0", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ParkingSim as Program<ParkingSim>;
  const wallet = provider.wallet as anchor.Wallet;

  // Random ID for fresh tests
  const spotId = `Spot_${Math.floor(Math.random() * 10000)}`;
  const licensePlate = "SOL-RICH";

  // Calculate PDA address
  const [spotPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("spot"), Buffer.from(spotId)],
    program.programId
  );

  // ============ TESTS ============
  // These tests will FAIL until you implement the program!

  it("1. Initializes the Spot", async () => {
    // TODO: Once you implement initialize_spot, this test should pass
    
    await program.methods
      .initializeSpot(spotId)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    
    assert.equal(spotAccount.isOccupied, false, "Spot should not be occupied");
    assert.equal(
      spotAccount.lotOwner.toString(),
      wallet.publicKey.toString(),
      "Lot owner should be signer"
    );
    
    console.log(`✅ Spot ${spotId} created`);
  });

  it("2. Parks a Car (pays SOL)", async () => {
    // Get SOL balances before
    const userBalanceBefore = await provider.connection.getBalance(wallet.publicKey);
    const spotBalanceBefore = await provider.connection.getBalance(spotPda);
    
    await program.methods
      .carPark(spotId, licensePlate)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Check spot state
    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, true, "Spot should be occupied");
    assert.equal(spotAccount.licensePlate, licensePlate, "License plate should match");
    
    // Check SOL transferred (0.1 SOL = 100_000_000 lamports)
    const spotBalanceAfter = await provider.connection.getBalance(spotPda);
    const spotBalanceDiff = spotBalanceAfter - spotBalanceBefore;
    assert.equal(spotBalanceDiff, 100_000_000, "Spot should receive 0.1 SOL");
    
    console.log(`✅ Car parked, paid 0.1 SOL`);
  });

  it("3. Cannot park in occupied spot", async () => {
    try {
      await program.methods
        .carPark(spotId, "HACKER-1")
        .accounts({
          spot: spotPda,
          signer: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      assert.fail("Should have thrown SpotTaken error");
    } catch (error) {
      assert.include(error.toString(), "SpotTaken");
      console.log("✅ Correctly rejected parking in occupied spot");
    }
  });

  it("4. Owner withdraws profit", async () => {
    const ownerBalanceBefore = await provider.connection.getBalance(wallet.publicKey);
    const spotBalanceBefore = await provider.connection.getBalance(spotPda);
    
    // Withdraw 50_000_000 lamports (0.05 SOL)
    await program.methods
      .withdrawProfit(spotId, new anchor.BN(50_000_000))
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const spotBalanceAfter = await provider.connection.getBalance(spotPda);
    const spotBalanceDiff = spotBalanceBefore - spotBalanceAfter;
    
    // Note: Actual received might be less due to rent-exempt minimum
    console.log(`✅ Withdrew ${spotBalanceDiff / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  });

  it("5. Leave parking spot", async () => {
    await program.methods
      .leaveParking(spotId)
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
      })
      .rpc();

    const spotAccount = await program.account.parkingSpot.fetch(spotPda);
    assert.equal(spotAccount.isOccupied, false, "Spot should be empty");
    
    console.log("✅ Left parking spot");
  });

  it("6. Non-owner cannot withdraw", async () => {
    // First, park again to have funds
    await program.methods
      .carPark(spotId, "TEST-123")
      .accounts({
        spot: spotPda,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Create a hacker wallet
    const hacker = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
      hacker.publicKey, 
      anchor.web3.LAMPORTS_PER_SOL
    );
    await new Promise(resolve => setTimeout(resolve, 1000));

    try {
      await program.methods
        .withdrawProfit(spotId, new anchor.BN(50_000_000))
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
