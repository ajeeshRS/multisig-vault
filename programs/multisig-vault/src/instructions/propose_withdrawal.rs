use std::vec;

use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::{
    MULTSIG_SEED, PROPOSER_SEED, VAULT_SEED, error::MultiSigVaultErrors, vault::{InstructionData, MultisigConfig, Proposal, StoredAccountMeta}
};

#[derive(Accounts)]
pub struct ProposeWithdrawal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(
        seeds = [MULTSIG_SEED.as_bytes(), multisig_config.creator.as_ref() ],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,

    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [PROPOSER_SEED.as_bytes(), multisig_config.key().as_ref(), multisig_config.proposal_count.to_le_bytes().as_ref() ],
        bump
    )]
    pub proposal_account: Account<'info, Proposal>,

    /// CHECK: vault
    #[account(
        seeds = [VAULT_SEED.as_bytes(),multisig_config.key().as_ref()],
        bump = multisig_config.vault_bump,
    )]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProposeWithdrawal<'info> {
    pub fn propose(&mut self, lamports: u64, recipient_addr: Pubkey) -> Result<()> {
        require!(
            self.multisig_config.owners.contains(&self.proposer.key()),
            MultiSigVaultErrors::ProposerNotOwner
        );

        require!(
            lamports <= self.vault.lamports(),
            MultiSigVaultErrors::InSufficientLamports
        );

        let approvals = vec![self.proposer.key()];

        let tfr_ix = system_instruction::transfer(&self.vault.key(), &recipient_addr, lamports);

        let ix_data = InstructionData {
            program_id: tfr_ix.program_id,
            accounts: tfr_ix.accounts.iter().map(|acc| {
                StoredAccountMeta {
                    pubkey: acc.pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable
                }
            }).collect(),
            data: tfr_ix.data,
        };

        self.proposal_account.set_inner(Proposal {
            index: self.multisig_config.proposal_count,
            proposer: self.proposer.key(),
            recipient: recipient_addr,
            multisig: self.multisig_config.key(),
            approvals: approvals,
            instructions: vec![ix_data],
            executed: false,
        });

        self.multisig_config.proposal_count += 1;
        Ok(())
    }
}


