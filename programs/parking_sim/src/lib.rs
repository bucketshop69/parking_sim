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
        // let spot = ...
        
        // TODO: Set the initial values
        // spot.is_occupied = ...
        // spot.lot_owner = ...
        // spot.car_owner = ...
        // spot.license_plate = ...
        
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
        // let spot = ...
        // let customer = ...
        
        // TODO: Check if spot is occupied
        // if ... {
        //     return err!(ErrorCode::SpotTaken);
        // }
        
        // TODO: Transfer SOL (100_000_000 lamports = 0.1 SOL)
        // HINT: Look up anchor_lang::system_program::Transfer
        // let transfer_instruction = Transfer { ... };
        // let cpi_ctx = CpiContext::new(...);
        // system_program::transfer(cpi_ctx, amount)?;
        
        // TODO: Update spot state
        // spot.is_occupied = ...
        // spot.license_plate = ...
        // spot.car_owner = ...
        
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
    pub fn withdraw_profit(ctx: Context<WithdrawProfit>, spot_id: String, amount: u64) -> Result<()> {
        // TODO: Verify lot owner
        // require!(spot.lot_owner == ..., ErrorCode::NotYourProfit);
        
        // TODO: Transfer SOL from spot to owner
        // HINT: For PDA signing, you need the seeds:
        // let bump = ctx.bumps.spot;
        // let seeds = [b"spot", spot_id.as_bytes(), &[bump]];
        // Then use invoke_signed or a different approach
        
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
        
        // TODO: Verify car owner
        
        // TODO: Clear spot state
        // spot.is_occupied = false;
        // spot.license_plate = "".to_string();
        // spot.car_owner = Pubkey::default();
        
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
    // TODO: Add account constraints for spot
    // #[account(
    //     init,
    //     payer = ???,
    //     space = ???,  // 8 (discriminator) + 1 (bool) + 32 (Pubkey) + 32 (Pubkey) + (4 + 20) (String)
    //     seeds = [b"spot", spot_id.as_bytes()],
    //     bump
    // )]
    // pub spot: Account<'info, ParkingSpot>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts needed to park a car
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct CarPark<'info> {
    // TODO: Add account for spot (mut, seeds, bump)
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts needed to withdraw profit
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct WithdrawProfit<'info> {
    // TODO: Add account for spot (mut, seeds, bump)
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts needed to leave parking
#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct LeaveParking<'info> {
    // TODO: Add account for spot (mut, seeds, bump)
    
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
    pub is_occupied: bool,      // Is a car parked here?
    pub license_plate: String,  // License plate of parked car
    pub lot_owner: Pubkey,      // Who owns this parking spot?
    pub car_owner: Pubkey,      // Who parked the car?
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
