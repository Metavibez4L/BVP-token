use anchor_lang::prelude::*;
use anchor_spl::token::Token;

const STAKE_STATE_SIZE: usize = 8 + 32 + 8 + 8 + 8; // discriminator + Pubkey + 3 u64s

#[account]
pub struct StakeState {
    pub user: Pubkey,
    pub amount: u64,
    pub start_timestamp: i64,
    pub duration: u64,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(init, payer = user, space = STAKE_STATE_SIZE)]
    pub stake_state: Account<'info, StakeState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64, duration: u64) -> Result<()> {
    let stake_state = &mut ctx.accounts.stake_state;
    stake_state.user = ctx.accounts.user.key();
    stake_state.amount = amount;
    stake_state.start_timestamp = Clock::get()?.unix_timestamp;
    stake_state.duration = duration;

    msg!("Staked {} tokens for {} seconds", amount, duration);
    Ok(())
}
