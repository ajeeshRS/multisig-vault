use anchor_lang::prelude::*;

use crate::{
    error::MultiSigVaultErrors, vault::MultisigConfig, MIN_OWNERS, MIN_THRESHOLD, MULTSIG_SEED,
    VAULT_SEED,
};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + MultisigConfig::INIT_SPACE,
        seeds = [MULTSIG_SEED.as_bytes(), creator.key().as_ref() ],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,

    /// CHECK: vault
    #[account(
        seeds = [VAULT_SEED.as_bytes(), multisig_config.key().as_ref()],
        bump,
    )]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn init(
        &mut self,
        owners: Vec<Pubkey>,
        threshold: u8,
        bumps: InitializeVaultBumps,
    ) -> Result<()> {
        // min owners check
        require!(
            owners.len() as u8 >= MIN_OWNERS,
            MultiSigVaultErrors::InvalidOwners
        );

        // creator is includes in owners check
        require!(
            owners.contains(&self.creator.key()),
            MultiSigVaultErrors::CreatorNotOwner
        );

        // threshold min check
        require!(
            threshold > MIN_THRESHOLD,
            MultiSigVaultErrors::InvalidThreshold
        );

        // threshold and ownerlen check
        require!(
            threshold <= owners.len() as u8,
            MultiSigVaultErrors::InvalidThreshold
        );

        let mut owners = owners;
        owners.sort();
        owners.dedup();

        // duplicat owners check
        require!(
            owners.len() == owners.len(),
            MultiSigVaultErrors::DuplicateOwners
        );

        self.multisig_config.set_inner(MultisigConfig {
            creator: self.creator.key(),
            proposal_count: 0,
            owners,
            threshold,
            vault: self.vault.key(),
            vault_bump: bumps.vault,
        });
        Ok(())
    }
}
