pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E346HcHEE6H7qzRqPqBmgkA7FQnD69s68r6ZmZkvVFt6");

#[program]
pub mod multisig_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        owners: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        ctx.accounts.init(owners, threshold, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn propose_withdrawal(
        ctx: Context<ProposeWithdrawal>,
        lamports: u64,
        recipient_addr: Pubkey,
    ) -> Result<()> {
        ctx.accounts.propose(lamports, recipient_addr)
    }
    pub fn approve_withdrawal(ctx: Context<ApproveWithdrawal>) -> Result<()> {
        ctx.accounts.approve()
    }
    pub fn execute_tx(ctx: Context<ExecuteTx>) -> Result<()> {
        ctx.accounts.execute()
    }
}
