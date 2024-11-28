use anchor_lang::prelude::*;
use anchor_spl::{
    token::{mint_to, MintTo},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"collection_mint".as_ref(), &[ctx.bumps.collection_mint]]];

    msg!("Creating mint accounts");

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.collection_token_account.to_account_info(),
                authority: ctx.accounts.collection_mint.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
    init,
    payer = authority,
    mint::decimals = 0,
    mint::authority = collection_mint,
    mint::freeze_authority = collection_mint,
    seeds = [b"collection_mint".as_ref()],
    bump
  )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
      init,
      payer = authority,
      token::mint = collection_mint,
      token::authority = collection_mint,
      seeds = [b"collection_token_account".as_ref()],
      bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}
