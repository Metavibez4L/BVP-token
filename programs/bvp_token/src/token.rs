use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = user)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<InitializeToken>, _supply: u64) -> Result<()> {
    msg!("Initializing token mint...");
    Ok(())
}
