use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint};

declare_id!("9erFjuSDr2NwCUGmd1H1epfhMcy7aEF7g5xtTn71Eh3m");

pub mod token;
pub mod staking;
pub mod multisig;
pub mod governance;
pub mod tiers;

use token::InitializeToken;
use staking::{Stake, ClaimRewards};
use multisig::{ApproveTx, MultiSigTx};
use governance::VoteGovernance;
use tiers::{CheckTier, Tier};

#[program]
pub mod bvp_token {
    use super::*;

    pub fn initialize(ctx: Context<InitializeToken>, total_supply: u64) -> Result<()> {
        token::initialize(ctx, total_supply)
    }

    pub fn stake_tokens(ctx: Context<Stake>, amount: u64, duration: u64) -> Result<()> {
        staking::stake_tokens(ctx, amount, duration)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        staking::claim_rewards(ctx)
    }

    pub fn execute_multisig_transaction(ctx: Context<MultiSigTx>) -> Result<()> {
        multisig::execute_multisig_transaction(ctx)
    }

    pub fn approve_transaction(ctx: Context<ApproveTx>) -> Result<()> {
        multisig::approve_transaction(ctx)
    }

    pub fn vote_governance(ctx: Context<VoteGovernance>, approve: bool) -> Result<()> {
        governance::vote_governance(ctx, approve)
    }

    pub fn check_tier(ctx: Context<CheckTier>, user: Pubkey) -> Result<Tier> {
        tiers::check_tier(ctx, user)
    }
}
