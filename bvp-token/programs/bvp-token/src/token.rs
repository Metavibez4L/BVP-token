use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

pub fn initialize(ctx: Context<InitializeToken>, total_supply: u64) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::mint_to(cpi_ctx, total_supply)?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
