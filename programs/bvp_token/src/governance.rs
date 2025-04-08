use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VoteGovernance<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub approvals: u8,
    pub rejections: u8,
    pub threshold: u8,
    pub executed: bool,
}

pub fn vote_governance(ctx: Context<VoteGovernance>, approve: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    require!(!proposal.executed, GovernanceError::ProposalAlreadyExecuted);

    if approve {
        proposal.approvals += 1;
    } else {
        proposal.rejections += 1;
    }

    if proposal.approvals >= proposal.threshold {
        proposal.executed = true;
        msg!("Proposal executed.");
    }

    Ok(())
}

#[error_code]
pub enum GovernanceError {
    #[msg("This proposal has already been executed.")]
    ProposalAlreadyExecuted,
}
