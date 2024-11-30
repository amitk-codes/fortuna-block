pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FoAMdZmnK7eTejEem6WJQUhVDkrDT3VsJuRu9WVQ5B7J");

#[program]
pub mod fortuna_block {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
    ) -> Result<()> {
        initialize_config::initialize_config_handler(ctx, start_time, end_time, ticket_price)
    }

    pub fn initialize_lottery_accounts(ctx: Context<InitializeLottery>) -> Result<()> {
        initialize_lottery::initialize_lottery_handler(ctx)
    }
}
