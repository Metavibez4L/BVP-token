use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint, Token, TokenAccount};

declare_id!("5rDtfenu18pntzG66DAo18sWs4oc7NqecRdtEcdjtCXu");

pub mod token;
pub mod staking;
pub mod multisig;
pub mod governance;
pub mod tiers;

use token::*;
use staking::*;
use governance::*;
use multisig::*;
use tiers::*;

#[program]
pub mod bvp_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        initialize_handler(ctx, total_supply)
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64, duration: u64) -> Result<()> {
        stake_tokens_handler(ctx, amount, duration)
    }
}
