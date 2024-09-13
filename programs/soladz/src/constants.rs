use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const APP_STATS_SEED: &[u8] = b"app-stats";
pub const INVESTOR_SEED: &[u8] = b"investor";
pub const VAULT_SEED: &[u8] = b"vault";
pub const WHALE_POOL_SEED: &[u8] = b"whale-pool";
pub const ADMIN_SEED: &[u8] = b"admin";
pub const SEVER_SIGNER: Pubkey = pubkey!("H9mVt3VUBUYADZKhw5rjPgEs57o877J9WF9QGrBVBfkq");
pub const ADMIN: Pubkey = pubkey!("2TKdkAQ8RLM8anLDJ2KBY3EmwswgHrhydpwyqk1qAocd");
pub const PERIOD: i64 = 86400;