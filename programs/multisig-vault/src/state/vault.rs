use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigConfig {
    pub creator: Pubkey,
    #[max_len(10)]
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub proposal_count: u64,
    pub vault: Pubkey,
    pub vault_bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub index: u64,
    pub proposer: Pubkey,
    pub recipient: Pubkey,
    pub multisig: Pubkey,
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,
    #[max_len(5)]
    pub instructions: Vec<InstructionData>,
    pub executed: bool,
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone,InitSpace)]
pub struct InstructionData {
    pub program_id: Pubkey,
    #[max_len(5)]
    pub accounts : Vec<StoredAccountMeta>,
    #[max_len(100)]
    pub data: Vec<u8>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct StoredAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}