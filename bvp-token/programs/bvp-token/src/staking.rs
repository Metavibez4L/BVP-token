use anchor_lang::prelude::*;

pub fn stake_tokens(ctx: Context<Stake>, amount: u64, duration: u64) -> Result<()> {
    let staker = &mut ctx.accounts.staker;
    staker.amount_staked = amount;
    staker.staking_end = Clock::get()?.unix_timestamp + duration as i64;
    staker.reward_claimed = false;
    Ok(())
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let staker = &mut ctx.accounts.staker;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time >= staker.staking_end,
        crate::ErrorCode::StakingPeriodNotOver
    );

    require!(!staker.reward_claimed, StakingError::AlreadyClaimed);

    staker.reward_claimed = true;
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8 + 8 + 1)]
    pub staker: Account<'info, Staker>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, has_one = user)]
    pub staker: Account<'info, Staker>,
    pub user: Signer<'info>,
}

#[account]
pub struct Staker {
    pub amount_staked: u64,
    pub staking_end: i64,
    pub reward_claimed: bool,
}

#[error_code]
pub enum StakingError {
    #[msg("Rewards already claimed.")]
    AlreadyClaimed,
}
