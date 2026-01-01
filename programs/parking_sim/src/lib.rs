use anchor_lang::prelude::*;

declare_id!("Dn6w2wuW9vsG9CS8Nc4ufqKnfWLS3wGcBLPJa48kcvG3");

#[program]
pub mod parking_sim {

    use super::*;

    pub fn initialize_lot_account(ctx: Context<InitializeLotAccount>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let parking_sim = &mut ctx.accounts.lot_account;

        parking_sim.owner = ctx.accounts.signer.key();
        parking_sim.level = 1;
        parking_sim.total_earned = 0;
        parking_sim.spots = [SpotState::Empty; 10];
        parking_sim.event_queue = Vec::new();
        parking_sim.upgrade_requested_at = 0;
        parking_sim.upgrade_paid = false;

        Ok(())
    }

    pub fn initialize_bot(
        ctx: Context<InitializeBot>,
        bot_index: u8,
        lotPDA: Pubkey,
    ) -> Result<()> {
        let bot_account = &mut ctx.accounts.bot_account;

        bot_account.index = bot_index;
        bot_account.lot = lotPDA;
        bot_account.bot_type = BotType::Basic;
        bot_account.total_visits = 0;
        bot_account.times_missed = 0;
        bot_account.status = BotStatus::Idle;
        bot_account.spot_index = Option::None;
        bot_account.total_paid = 0;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(bot_index: u8, lotPDA: Pubkey)]
pub struct InitializeBot<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8+32+1+1+4+8+4+1+2,
        seeds = [b"bot_account", lotPDA.as_ref(), &[bot_index]],
        bump
    )]
    bot_account: Account<'info, BotAccount>,

    system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitializeLotAccount<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8+32+1+8+20+204+8+1,
        seeds = [b"parking_sim", signer.key().as_ref()],
        bump
    )]
    lot_account: Account<'info, LotAccount>,

    system_program: Program<'info, System>,
}

#[account]
pub struct LotAccount {
    owner: Pubkey,             // 32
    level: u8,                 // 1
    total_earned: u64,         // 8
    spots: [SpotState; 10],    // 10 * 2 = 20
    event_queue: Vec<Event>,   // 4 (20*10) = 204
    upgrade_requested_at: i64, // 8
    upgrade_paid: bool,        // 1
}

#[account]
pub struct BotAccount {
    pub lot: Pubkey,            // 32
    pub index: u8,              // 1
    pub bot_type: BotType,      // 1
    pub total_visits: u32,      // 4
    pub total_paid: u64,        // 8
    pub times_missed: u32,      // 4
    pub status: BotStatus,      // 1
    pub spot_index: Option<u8>, // 1 + 1 = 2
}

// SpotState: Is a parking spot empty or occupied?
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpotState {
    #[default]
    Empty, // No one parked here
    Occupied {
        bot_index: u8,
    }, // Bot #X is parked here
}

// BotStatus: Where is the bot right now?
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum BotStatus {
    #[default]
    Idle, // At home, waiting for next arrival time
    Waiting, // At your lot, patience timer ticking
    Parked,  // In a spot, stay timer ticking
}

// BotType: Basic or VIP?
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum BotType {
    #[default]
    Basic, // 5 $PARK, 60s patience
    Vip, // 25 $PARK, 10s patience
}

// EventType: What kind of scheduled event?
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Arrival,  // Bot shows up at lot
    Patience, // Bot's patience runs out
    Checkout, // Bot's parking time ends
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Event {
    pub event_type: EventType, // 1 byte
    pub bot_index: u8,         // 1 byte
    pub timestamp: i64,        // 8 bytes
}
