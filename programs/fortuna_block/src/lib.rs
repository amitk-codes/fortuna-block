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

    pub fn initialize(
        ctx: Context<InitializeLottery>,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
    ) -> Result<()> {
        initialize::initialize_lottery(ctx, start_time, end_time, ticket_price)
    }
}
