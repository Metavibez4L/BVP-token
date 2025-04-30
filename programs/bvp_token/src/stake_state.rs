use anchor_lang::prelude::*;

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
}

pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64, duration: u64) -> Result<()> {
    let st = &mut ctx.accounts.stake_state;
    st.user = ctx.accounts.user.key();
    st.amount = amount;
    st.start_timestamp = Clock::get()?.unix_timestamp;
    st.duration = duration;
    msg!("Staked {} for {}s", amount, duration);
    Ok(())
}

//
// --- NEW: unstake instruction + context ---
//
#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(mut, has_one = user)]
    pub stake_state: Account<'info, StakeState>,
    pub user: Signer<'info>,
}

pub fn unstake_tokens(ctx: Context<UnstakeTokens>) -> Result<()> {
    let st = &mut ctx.accounts.stake_state;
    // Simply zero out the amount on unstake
    st.amount = 0;
    msg!("Unstaked — amount reset to 0");
    Ok(())
}
