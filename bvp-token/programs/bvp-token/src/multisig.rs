use anchor_lang::prelude::*;

pub fn approve_transaction(ctx: Context<ApproveTx>) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;

    require!(!transaction.executed, crate::ErrorCode::TransactionAlreadyExecuted);

    transaction.approvals += 1;

    Ok(())
}

pub fn execute_multisig_transaction(ctx: Context<MultiSigTx>, _tx_id: u64) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;

    require!(!transaction.executed, crate::ErrorCode::TransactionAlreadyExecuted);
    require!(
        transaction.approvals >= transaction.threshold,
        crate::ErrorCode::InsufficientApprovals
    );

    transaction.executed = true;

    Ok(())
}

#[derive(Accounts)]
pub struct ApproveTx<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
}

#[derive(Accounts)]
pub struct MultiSigTx<'info> {
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
}

#[account]
pub struct Transaction {
    pub approvals: u8,
    pub threshold: u8,
    pub executed: bool,
}
