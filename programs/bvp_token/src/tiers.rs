use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CheckTier<'info> {
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Tier {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

pub fn check_tier(_ctx: Context<CheckTier>, _user: Pubkey) -> Result<Tier> {
    let token_amount: u64 = 1_000_000;

    let tier = if token_amount >= 2_000_000 {
        Tier::Diamond
    } else if token_amount >= 1_000_000 {
        Tier::Platinum
    } else if token_amount >= 500_000 {
        Tier::Gold
    } else if token_amount >= 100_000 {
        Tier::Silver
    } else if token_amount >= 10_000 {
        Tier::Bronze
    } else {
        Tier::None
    };

    Ok(tier)
}
