use anchor_lang::prelude::*;

#[constant]
pub const MULTSIG_SEED: &str = "multisig_config";
#[constant]
pub const VAULT_SEED: &str = "vault";
#[constant]
pub const PROPOSER_SEED: &str = "proposer";

#[constant]
pub const MIN_OWNERS: u8 = 3;
#[constant]
pub const MIN_THRESHOLD: u8 = 3;
