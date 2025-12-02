use anchor_lang::prelude::*;

declare_id!("Dn6w2wuW9vsG9CS8Nc4ufqKnfWLS3wGcBLPJa48kcvG3");

// ============================================================================
// LEVEL 0: Basic Parking Lot with SOL Payments
// ============================================================================
//
// YOUR MISSION:
// Build a simple parking lot system where:
// 1. Lot owner can create parking spots
// 2. Users can park their car (pay SOL)
// 3. Lot owner can withdraw profits
// 4. Users can leave the parking spot
//
// CONCEPTS YOU'LL LEARN:
// - Program structure
// - PDAs (Program Derived Addresses)
// - Account validation
// - SOL transfers via CPI
// - Error handling
//
// ============================================================================

#[program]
pub mod parking_sim {
    use anchor_lang::system_program;
    use anchor_lang::system_program::Transfer;

    use super::*;

    /// Initialize a new parking spot
    ///
    /// TODO: Implement this function
    /// 1. Set is_occupied to false
    /// 2. Set lot_owner to the signer's public key
    /// 3. Set car_owner to Pubkey::default()
    /// 4. Log a message confirming spot creation
    pub fn initialize_spot(ctx: Context<InitSpot>, spot_id: String) -> Result<()> {
        msg!("Creating spot: {}", spot_id);

        // TODO: Get a mutable reference to the spot account
        let spot = &mut ctx.accounts.spot;

        // TODO: Set the initial values
        spot.is_occupied = false;
        spot.lot_owner = ctx.accounts.signer.key();
        spot.car_owner = Pubkey::default();
        spot.license_plate = "".to_string();

        Ok(())
    }

    /// Park a car in a spot
    ///
    /// TODO: Implement this function
    /// 1. Check if spot is already occupied (return error if so)
    /// 2. Transfer SOL from user to the spot PDA
    /// 3. Update spot state (is_occupied, license_plate, car_owner)
    ///
    /// HINT: For SOL transfer, you need:
    /// - Create a Transfer struct with from/to accounts
    /// - Create a CpiContext
    /// - Call system_program::transfer()
    pub fn car_park(ctx: Context<CarPark>, spot_id: String, license_plate: String) -> Result<()> {
        msg!("Parking car in spot: {}", spot_id);

        // TODO: Get references to accounts
        let spot = &mut ctx.accounts.spot;
        let customer = &mut ctx.accounts.signer;

        // TODO: Check if spot is occupied
        if spot.is_occupied == true {
            return err!(ErrorCode::SpotTaken);
        }

        // TODO: Transfer SOL (100_000_000 lamports = 0.1 SOL)
        // HINT: Look up anchor_lang::system_program::Transfer
        let transfer_instruction = Transfer {
            from: customer.to_account_info(),
            to: spot.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        system_program::transfer(cpi_ctx, 100_000_000)?;

        // TODO: Update spot state
        spot.is_occupied = true;
        spot.license_plate = license_plate;
        spot.car_owner = customer.key();

        Ok(())
    }

    /// Withdraw profits from a parking spot
    ///
    /// TODO: Implement this function
    /// 1. Verify the signer is the lot owner
    /// 2. Transfer SOL from spot PDA back to owner
    ///
    /// HINT: The spot PDA needs to "sign" the transfer.
    /// Since it's a PDA, your program can sign for it!
    /// Use the spot's seeds to sign.
    pub fn withdraw_profit(
        ctx: Context<WithdrawProfit>,
        spot_id: String,
        amount: u64,
    ) -> Result<()> {
        let signer = &ctx.accounts.signer;
        let spot = &ctx.accounts.spot;

        require!(spot.lot_owner == signer.key(), ErrorCode::NotYourProfit);

        // NOTE: Cannot use System Program transfer from a PDA that holds data!
        // The System Program requires `from` account to have no data.
        // Instead, we directly manipulate lamports (safe because we own this PDA).

        let spot_info = spot.to_account_info();
        let owner_info = signer.to_account_info();

        **spot_info.try_borrow_mut_lamports()? -= amount;
        **owner_info.try_borrow_mut_lamports()? += amount;

        msg!("Withdrew {} lamports from spot: {}", amount, spot_id);
        Ok(())
    }
    /// Leave a parking spot
    ///
    /// TODO: Implement this function
    /// 1. Verify the signer is the car owner
    /// 2. Clear the spot state
    pub fn leave_parking(ctx: Context<LeaveParking>, spot_id: String) -> Result<()> {
        msg!("Leaving spot: {}", spot_id);

        let spot = &mut ctx.accounts.spot;
        let signer = &mut ctx.accounts.signer;
        // TODO: Verify car owner
        if spot.car_owner != signer.key() {
            return err!(ErrorCode::NotYourCar);
        }

        spot.is_occupied = false;
        spot.license_plate = "".to_string();
        spot.car_owner = Pubkey::default();

        Ok(())
    }
}

// ============================================================================
// ACCOUNT STRUCTS - Define what accounts each instruction needs
// ============================================================================

/// Accounts needed to initialize a parking spot
///
/// TODO: Add the correct constraints
/// HINTS:
/// - spot needs: init, payer, space calculation, seeds, bump
/// - signer needs: mut (because they pay)
/// - system_program: needed for account creation
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct InitSpot<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 1 + 32 + 32 + (4+20),  // 8 (discriminator) + 1 (bool) + 32 (Pubkey) + 32 (Pubkey) + (4 + 20) (String)
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts needed to park a car
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct CarPark<'info> {
    // TODO: Add account for spot (mut, seeds, bump)
    #[account(
        mut,
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts needed to withdraw profit
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct WithdrawProfit<'info> {
    #[account(
        mut,
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts needed to leave parking
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct LeaveParking<'info> {
    // TODO: Add account for spot (mut, seeds, bump)
    #[account(
        mut,
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,

    pub signer: Signer<'info>,
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// The parking spot account data
///
/// This struct defines what data is stored in each parking spot
#[account]
pub struct ParkingSpot {
    pub is_occupied: bool,     // Is a car parked here?
    pub license_plate: String, // License plate of parked car
    pub lot_owner: Pubkey,     // Who owns this parking spot?
    pub car_owner: Pubkey,     // Who parked the car?
}

// ============================================================================
// ERRORS
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Spot is already occupied.")]
    SpotTaken,

    #[msg("You do not own the car in this spot.")]
    NotYourCar,

    #[msg("You are not the lot owner")]
    NotYourProfit,
}
