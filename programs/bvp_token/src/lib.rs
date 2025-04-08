use anchor_lang::prelude::*;
use anchor_spl::token::{self as spl_token, Mint};

pub mod token;
pub mod staking;
pub mod governance;
pub mod multisig;
pub mod tiers;

// Just load the token module for now
use token::*;

declare_id!("9erFjuSDr2NwCUGmd1H1epfhMcy7aEF7g5xtTn71Eh3m");

#[program]
pub mod bvp_token {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, _total_supply: u64) -> Result<()> {
        Ok(())
    }

    pub fn stake_tokens(_ctx: Context<Stake>, _amount: u64, _duration: u64) -> Result<()> {
        Ok(())
    }
}
