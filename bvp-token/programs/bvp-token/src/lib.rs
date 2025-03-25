use anchor_lang::prelude::*;

pub mod token;
pub mod staking;
pub mod multisig;
pub mod governance;
pub mod tiers;

use token::*;
use staking::*;
use multisig::*;
use governance::*;
use tiers::*;

#[program]
pub mod bvp_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        token::initialize(ctx, total_supply)
    }

    pub fn stake_tokens(ctx: Context<Stake>, amount: u64, duration: u64) -> Result<()> {
        staking::stake_tokens(ctx, amount, duration)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        staking::claim_rewards(ctx)
    }

    pub fn execute_multisig_transaction(ctx: Context<MultiSigTx>, tx_id: u64) -> Result<()> {
        multisig::execute_multisig_transaction(ctx, tx_id)
    }

    pub fn approve_transaction(ctx: Context<ApproveTx>) -> Result<()> {
        multisig::approve_transaction(ctx)
    }

    pub fn vote_governance(ctx: Context<VoteGovernance>, proposal_id: u64, approve: bool) -> Result<()> {
        governance::vote_governance(ctx, proposal_id, approve)
    }

    pub fn check_tier(ctx: Context<CheckTier>, user: Pubkey) -> Result<Tier> {
        tiers::check_tier(ctx, user)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority)]
    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Staking period is not over yet.")]
    StakingPeriodNotOver,
    #[msg("Transaction already executed.")]
    TransactionAlreadyExecuted,
    #[msg("Not enough approvals for execution.")]
    InsufficientApprovals,
}
