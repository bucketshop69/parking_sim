use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;

declare_id!("Dn6w2wuW9vsG9CS8Nc4ufqKnfWLS3wGcBLPJa48kcvG3");

#[program]
pub mod parking_sim {

    use super::*;

    pub fn initialize_spot(ctx: Context<InitSpot>, spot_id: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let parking_spot = &mut ctx.accounts.spot;
        let signer = &mut ctx.accounts.signer;

        parking_spot.is_occupied = false;
        parking_spot.lot_owner = *signer.key;
        parking_spot.car_owner = Pubkey::default();
        msg!("spot created for {}", spot_id);

        Ok(())
    }

    pub fn car_park(ctx: Context<CarPark>, spot_id: String, license_plate: String) -> Result<()> {
        msg!("Park your car: {}", spot_id);

        let spot = &mut ctx.accounts.spot;
        let customer: &mut Signer<'_> = &mut ctx.accounts.signer;
        if spot.is_occupied == true {
            return err!(ErrorCode::SpotTaken);
        }

        // --- PAYMENT ---
        let transfer_instruction = Transfer {
            from: customer.to_account_info(),
            to: spot.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );

        anchor_lang::system_program::transfer(cpi_ctx, 100_000_000)?;

        spot.is_occupied = true;
        spot.license_plate = license_plate;
        spot.car_owner = *customer.key;

        Ok(())
    }

    pub fn withdraw_profit(
        ctx: Context<WithdrawProfit>,
        spot_id: String,
        amount: u64,
    ) -> Result<()> {
        let spot = &mut ctx.accounts.spot;
        let manager = &mut ctx.accounts.signer;

        require!(spot.lot_owner == *manager.key, ErrorCode::NotYourProfit);

        // Check rent exemption
        let rent = Rent::get()?;
        let spot_info = spot.to_account_info();
        let min_balance = rent.minimum_balance(spot_info.data_len());

        require!(
            spot_info.lamports().saturating_sub(amount) >= min_balance,
            ErrorCode::InsufficientBalance
        );

        // ✅ Manually transfer lamports (System Program transfer won't work for PDAs with data)
        **spot_info.try_borrow_mut_lamports()? -= amount;
        **manager.to_account_info().try_borrow_mut_lamports()? += amount;

        msg!("Withdrew {} lamports from spot: {}", amount, spot_id);
        Ok(())
    }
    pub fn leave_parking(ctx: Context<LeaveParking>, spot_id: String) -> Result<()> {
        msg!("Leave car parking: {}", spot_id);

        let spot = &mut ctx.accounts.spot;

        if spot.lot_owner != *ctx.accounts.signer.key {
            return err!(ErrorCode::NotYourCar);
        }

        spot.is_occupied = false;
        spot.license_plate = "".to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct InitSpot<'info> {
    #[account(
        init,
        payer=signer,
        space= 8 + 1 + 32 + 32 + (4 + 20),
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct CarPark<'info> {
    #[account(
        mut,
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

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

#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct LeaveParking<'info> {
    #[account(
        mut,
        seeds = [b"spot", spot_id.as_bytes()],
        bump
    )]
    pub spot: Account<'info, ParkingSpot>,
    pub signer: Signer<'info>,
}

#[account]
pub struct ParkingSpot {
    pub is_occupied: bool,
    pub license_plate: String,
    pub lot_owner: Pubkey,
    pub car_owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Spot is already occupied.")]
    SpotTaken,
    #[msg("You do not own the car in this spot.")]
    NotYourCar,

    #[msg("You are not the lot owner")]
    NotYourProfit,

    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,
}
