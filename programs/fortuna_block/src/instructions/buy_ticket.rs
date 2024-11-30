use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{error::CustomError, TokenLottery};

pub fn buy_ticket_handler(ctx: Context<BuyTicket>) -> Result<()> {
    let clock = Clock::get()?;
    if clock.slot < ctx.accounts.token_lottery.start_time
        || clock.slot > ctx.accounts.token_lottery.end_time
    {
        return Err(CustomError::LotteryNotOpen.into());
    };

    transfer(
        CpiContext::new(
          ctx.accounts.system_program.to_account_info(),
          Transfer{
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.token_lottery.to_account_info(),
          }
        ),
        ctx.accounts.token_lottery.ticket_price,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = [b"token_lottery".as_ref()],
    bump
  )]
    pub token_lottery: Account<'info, TokenLottery>,

    pub system_program: Program<'info, System>,
}
