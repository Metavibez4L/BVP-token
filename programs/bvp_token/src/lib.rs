use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Mint, Transfer};
use anchor_lang::solana_program::clock::Clock;

declare_id!("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");

#[program]
pub mod bvp_token {
    use super::*;

    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        amount: u64,
        duration: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.stake_state;
        state.user = *ctx.accounts.user.key;
        state.amount = amount;
        state.start_timestamp = Clock::get()?.unix_timestamp;
        state.duration = duration;
        msg!("Staked {} tokens for {} seconds", amount, duration);
        Ok(())
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>) -> Result<()> {
        let state = &mut ctx.accounts.stake_state;
        state.amount = 0;
        msg!("Unstaked all tokens");
        Ok(())
    }
}

//
// -- account data & context definitions pulled into the root --
//

#[account]
pub struct StakeState {
    pub user: Pubkey,
    pub amount: u64,
    pub start_timestamp: i64,
    pub duration: u64,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8)]
    pub stake_state: Account<'info, StakeState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(mut, has_one = user)]
    pub stake_state: Account<'info, StakeState>,
    pub user: Signer<'info>,
}
