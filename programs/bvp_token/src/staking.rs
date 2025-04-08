use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn stake_tokens(_ctx: Context<Stake>, _amount: u64, _duration: u64) -> Result<()> {
    msg!("Staking tokens...");
    Ok(())
}

pub fn claim_rewards(_ctx: Context<ClaimRewards>) -> Result<()> {
    msg!("Claiming rewards...");
    Ok(())
}
