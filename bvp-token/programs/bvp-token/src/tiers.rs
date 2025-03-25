use anchor_lang::prelude::*;

pub fn check_tier(_ctx: Context<CheckTier>, amount: Pubkey) -> Result<Tier> {
    // Placeholder logic — in a real system, you'd read token balance from a token account
    // and classify based on real balances from SPL Token program
    // For now, we simulate tier determination externally

    let token_amount = 1_000_000; // TODO: fetch real balance via CPI

    let tier = if token_amount >= 2_000_000 {
        Tier::Diamond
    } else if token_amount >= 1_000_000 {
        Tier::Platinum
    } else if token_amount >= 500_000 {
        Tier::Gold
    } else if token_amount >= 100_000 {
        Tier::Silver
    } else if token_amount >= 20_000 {
        Tier::Bronze
    } else {
        Tier::None
    };

    Ok(tier)
}

#[derive(Accounts)]
pub struct CheckTier<'info> {
    // Placeholder: you may want to pass in a token account and use anchor_spl::token::TokenAccount
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
