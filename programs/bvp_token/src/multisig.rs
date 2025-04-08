use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MultiSigTx<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveTx<'info> {
    #[account(mut)]
    pub approver: Signer<'info>,
}

pub fn execute_multisig_transaction(_ctx: Context<MultiSigTx>) -> Result<()> {
    msg!("Executing multisig tx...");
    Ok(())
}

pub fn approve_transaction(_ctx: Context<ApproveTx>) -> Result<()> {
    msg!("Approving multisig tx...");
    Ok(())
}
