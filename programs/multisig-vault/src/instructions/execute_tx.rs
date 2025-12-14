use std::vec;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;

use crate::error::MultiSigVaultErrors;
use crate::vault::{MultisigConfig, Proposal};
use crate::{MULTSIG_SEED, PROPOSER_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct ExecuteTx<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,

    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(
        seeds = [MULTSIG_SEED.as_bytes(), multisig_config.creator.as_ref() ],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,

    #[account(
        seeds = [PROPOSER_SEED.as_bytes(), multisig_config.key().as_ref(), proposal_account.index.to_le_bytes().as_ref() ],
        bump
    )]
    pub proposal_account: Account<'info, Proposal>,

    /// CHECK: vault
    #[account(
        mut,
        seeds = [VAULT_SEED.as_bytes(),multisig_config.key().as_ref()],
        bump = multisig_config.vault_bump,
    )]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecuteTx<'info> {
    pub fn execute(&mut self) -> Result<()> {
        require!(
            self.recipient.key() == self.proposal_account.recipient,
            MultiSigVaultErrors::InvalidRecipient
        );

        require!(
            !self.proposal_account.executed,
            MultiSigVaultErrors::TxAlreadyExecuted
        );

        let approvals = self.proposal_account.approvals.len();

        let threshold = self.multisig_config.threshold;

        require!(
            approvals >= threshold.into(),
            MultiSigVaultErrors::ThresholdNotReached
        );

        let seeds = &[
            VAULT_SEED.as_bytes(),
            self.multisig_config.creator.as_ref(),
            &[self.multisig_config.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ix_data = &self.proposal_account.instructions[0];

        let metas: Vec<AccountMeta> = ix_data
            .accounts
            .iter()
            .map(|acc| {
                if acc.pubkey == self.vault.key() {
                    AccountMeta::new(acc.pubkey, true)
                } else if acc.is_writable {
                    AccountMeta::new(acc.pubkey, false)
                } else {
                    AccountMeta::new_readonly(acc.pubkey, acc.is_signer)
                }
            })
            .collect();

        let ix = Instruction {
            program_id: ix_data.program_id,
            accounts: metas,
            data: ix_data.data.clone(),
        };

        let account_infos = vec![
            self.vault.to_account_info(),
            self.recipient.to_account_info(),
            self.system_program.to_account_info(),
        ];

        invoke_signed(&ix, &account_infos, signer_seeds)?;

        self.proposal_account.executed = true;

        Ok(())
    }
}
