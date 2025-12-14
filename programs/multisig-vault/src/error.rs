use anchor_lang::prelude::*;

#[error_code]
pub enum MultiSigVaultErrors {
    #[msg("Tx already executed")]
    TxAlreadyExecuted,
    #[msg("Multi sig threshold not reached")]
    ThresholdNotReached,
    #[msg("Invalid Threshold")]
    InvalidThreshold,
    #[msg("Duplicate owners found")]
    DuplicateOwners,
    #[msg("Couldn't satisfy the min owner length")]
    InvalidOwners,
    #[msg("Creator not in owners")]
    CreatorNotOwner,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Proposer need to be an owner")]
    ProposerNotOwner,
    #[msg("Approver need to be an owner")]
    ApproverNotOwner,
    #[msg("Insufficient lamports")]
    InSufficientLamports,
    #[msg("Approval already give")]
    ApprovalAlreadyGiven,
    #[msg("Invalid recipient")]
    InvalidRecipient,
}
