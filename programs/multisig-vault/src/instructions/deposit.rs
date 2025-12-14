use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::MultiSigVaultErrors, vault::MultisigConfig, MULTSIG_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [MULTSIG_SEED.as_bytes(), multisig_config.creator.as_ref()],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,

    /// CHECK: vault
    #[account(
        seeds = [VAULT_SEED.as_bytes(),multisig_config.key().as_ref()],
        bump = multisig_config.vault_bump,
    )]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, MultiSigVaultErrors::InvalidAmount);

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.depositor.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
