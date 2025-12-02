use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};
declare_id!("Dn6w2wuW9vsG9CS8Nc4ufqKnfWLS3wGcBLPJa48kcvG3");

#[program]
pub mod parking_sim {

    use super::*;

    pub fn initialize_spot(
        ctx: Context<InitSpot>,
        spot_id: String,
        hourly_rate: u64,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let parking_spot = &mut ctx.accounts.spot;
        let signer = &mut ctx.accounts.signer;

        parking_spot.is_occupied = false;
        parking_spot.lot_owner = *signer.key;
        parking_spot.car_owner = Pubkey::default();
        parking_spot.parked_at = 0;
        parking_spot.hourly_rate = hourly_rate;
        msg!("spot created for {}", spot_id);

        Ok(())
    }

    pub fn car_park(ctx: Context<CarPark>, spot_id: String, license_plate: String) -> Result<()> {
        msg!("Park your car: {}", spot_id);

        let spot = &mut ctx.accounts.spot;
        let customer = &ctx.accounts.signer;

        if spot.is_occupied {
            return err!(ErrorCode::SpotTaken);
        }

        // Get current timestamp
        let clock = Clock::get()?;

        // Record parking start time
        spot.is_occupied = true;
        spot.license_plate = license_plate;
        spot.car_owner = *customer.key;
        spot.parked_at = clock.unix_timestamp;

        msg!("Car parked at timestamp: {}", spot.parked_at);
        Ok(())
    }
    pub fn withdraw_profit(
        ctx: Context<WithdrawProfit>,
        spot_id: String,
        amount: u64,
    ) -> Result<()> {
        let spot = &mut ctx.accounts.spot;

        require!(
            spot.lot_owner == *ctx.accounts.signer.key,
            ErrorCode::NotYourProfit
        );

        // --- TOKEN TRANSFER (Spot ATA → Owner ATA) ---
        let transfer_accounts = anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.spot_payment_ata.to_account_info(),
            to: ctx.accounts.owner_payment_ata.to_account_info(),
            mint: ctx.accounts.payment_mint.to_account_info(),
            authority: spot.to_account_info(), // The spot PDA is the authority!
        };

        // Sign with the spot PDA seeds
        let bump = ctx.bumps.spot;
        let seeds: &[&[&[u8]]] = &[&[b"spot", spot_id.as_bytes(), &[bump]]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            seeds,
        );

        let decimals = ctx.accounts.payment_mint.decimals;
        anchor_spl::token_interface::transfer_checked(transfer_ctx, amount, decimals)?;

        msg!("Withdrew {} tokens from spot: {}", amount, spot_id);
        Ok(())
    }

    pub fn leave_parking(ctx: Context<LeaveParking>, spot_id: String) -> Result<()> {
        let spot = &mut ctx.accounts.spot;
        let customer = &ctx.accounts.signer;

        // Only the car owner can leave (and pay)
        require!(spot.car_owner == *customer.key, ErrorCode::NotYourCar);
        require!(spot.is_occupied, ErrorCode::SpotNotOccupied);

        // Calculate duration
        let clock = Clock::get()?;
        let duration_seconds = clock.unix_timestamp - spot.parked_at;
        let duration_hours = (duration_seconds as u64) / 3600;

        // Minimum 1 hour charge
        let billable_hours = if duration_hours == 0 {
            1
        } else {
            duration_hours
        };

        // Calculate fee
        let fee = billable_hours * spot.hourly_rate;
        msg!("Parked for {} hours. Fee: {} tokens", billable_hours, fee);

        // --- TOKEN PAYMENT ---
        let transfer_accounts = anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.user_payment_ata.to_account_info(),
            to: ctx.accounts.spot_payment_ata.to_account_info(),
            mint: ctx.accounts.payment_mint.to_account_info(),
            authority: customer.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        let decimals = ctx.accounts.payment_mint.decimals;
        anchor_spl::token_interface::transfer_checked(transfer_ctx, fee, decimals)?;

        // --- MINT PARK REWARD (1 PARK per hour) ---
        let mint_accounts = MintTo {
            mint: ctx.accounts.park_mint.to_account_info(),
            to: ctx.accounts.user_park_ata.to_account_info(),
            authority: ctx.accounts.park_mint.to_account_info(),
        };

        let bump = ctx.bumps.park_mint;
        let seeds: &[&[&[u8]]] = &[&[b"park_mint", &[bump]]];

        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_accounts,
            seeds,
        );

        // Reward: 1 PARK per hour parked
        let reward = billable_hours * 10u64.pow(9); // 1 PARK per hour
        mint_to(mint_ctx, reward)?;

        msg!("Rewarded {} PARK tokens", billable_hours);

        // Clear the spot
        spot.is_occupied = false;
        spot.license_plate = "".to_string();
        spot.car_owner = Pubkey::default();
        spot.parked_at = 0;

        Ok(())
    }
    pub fn initialize_park_mint(_ctx: Context<InitializeParkMint>) -> Result<()> {
        msg!("PARK token mint initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(spot_id: String)]
pub struct InitSpot<'info> {
    #[account(
        init,
        payer=signer,
        space= 8 + 1 + 32 + 32 + (4 + 20)+ 8 + 8,
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

    #[account(mut)]
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

    /// Which token to withdraw
    pub payment_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = spot,
    )]
    pub spot_payment_ata: InterfaceAccount<'info, TokenAccount>,

    /// Owner's token account (where tokens go)
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = payment_mint,
        associated_token::authority = signer,
    )]
    pub owner_payment_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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

    #[account(mut)]
    pub signer: Signer<'info>,

    // Payment token
    pub payment_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = signer,
    )]
    pub user_payment_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = payment_mint,
        associated_token::authority = spot,
    )]
    pub spot_payment_ata: InterfaceAccount<'info, TokenAccount>,

    // PARK reward
    #[account(
        mut,
        seeds = [b"park_mint"],
        bump
    )]
    pub park_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = park_mint,
        associated_token::authority = signer,
    )]
    pub user_park_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitializeParkMint<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"park_mint"],
        bump,
        mint::decimals = 9,
        mint::authority = park_mint,
        mint::token_program = token_program,
    )]
    pub park_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
#[account]
pub struct ParkingSpot {
    pub is_occupied: bool,
    pub license_plate: String,
    pub lot_owner: Pubkey,
    pub car_owner: Pubkey,
    pub parked_at: i64,   // NEW: Unix timestamp when parked
    pub hourly_rate: u64, // NEW: Rate per hour (in smallest token units)
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

    #[msg("Spot is not occupied")] // NEW
    SpotNotOccupied,
}
