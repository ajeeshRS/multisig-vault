use std::vec;

use anchor_lang::prelude::*;

use crate::{
    error::MultiSigVaultErrors,
    vault::{MultisigConfig, Proposal},
    MULTSIG_SEED, PROPOSER_SEED, VAULT_SEED,
};

#[derive(Accounts)]
pub struct ApproveWithdrawal<'info> {
    #[account(mut)]
    pub approver: Signer<'info>,

    #[account(
        seeds = [MULTSIG_SEED.as_bytes(), multisig_config.creator.as_ref() ],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,

    #[account(
        mut,
        seeds = [PROPOSER_SEED.as_bytes(), multisig_config.key().as_ref(), proposal_account.index.to_le_bytes().as_ref() ],
        bump
    )]
    pub proposal_account: Account<'info, Proposal>,
}

impl<'info> ApproveWithdrawal<'info> {
    pub fn approve(&mut self) -> Result<()> {
        require!(
            self.multisig_config.owners.contains(&self.approver.key()),
            MultiSigVaultErrors::ApproverNotOwner
        );

        require!(
            !self.proposal_account.executed,
            MultiSigVaultErrors::TxAlreadyExecuted
        );

        require!(
            !self
                .proposal_account
                .approvals
                .contains(&self.approver.key()),
            MultiSigVaultErrors::ApprovalAlreadyGiven
        );

        self.proposal_account.approvals.push(self.approver.key());

        Ok(())
    }
}
