use anchor_lang::prelude::*;

pub mod stake_state;
use stake_state::*;

declare_id!("DGsg8rU5S9EQMYZaZgxJ2zihAX39jSd6ESnfZ5MQePQk");

#[program]
pub mod bvp_token {
    use super::*;

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64, duration: u64) -> Result<()> {
        stake_state::stake_tokens(ctx, amount, duration)
    }
}
