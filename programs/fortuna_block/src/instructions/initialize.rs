use anchor_lang::prelude::*;

use crate::state::TokenLottery;

pub fn initialize_lottery(
    ctx: Context<InitializeLottery>,
    start_time: u64,
    end_time: u64,
    ticket_price: u64,
) -> Result<()> {
    ctx.accounts.token_lottery.winner_claimed = false;
    ctx.accounts.token_lottery.start_time = start_time;
    ctx.accounts.token_lottery.end_time = end_time;
    ctx.accounts.token_lottery.lottery_pot_amount = 0;
    ctx.accounts.token_lottery.ticket_price = ticket_price;
    ctx.accounts.token_lottery.total_tickets = 0;
    ctx.accounts.token_lottery.authority = ctx.accounts.authority.key();
    ctx.accounts.token_lottery.randomness_account = Pubkey::default();
    ctx.accounts.token_lottery.bump = ctx.bumps.token_lottery;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + TokenLottery::INIT_SPACE, 
        seeds = [b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,

    pub system_program: Program<'info, System>,
}
