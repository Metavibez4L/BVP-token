use anchor_lang::prelude::*;

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
