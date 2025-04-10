use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = user)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_handler(
    ctx: Context<Initialize>,
    _total_supply: u64,
) -> Result<()> {
    // Mint initialization logic here
    Ok(())
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn stake_tokens_handler(
    ctx: Context<StakeTokens>,
    amount: u64,
    duration: u64,
) -> Result<()> {
    msg!("Staking {} tokens for {} seconds", amount, duration);
    // Add staking logic here
    Ok(())
}
