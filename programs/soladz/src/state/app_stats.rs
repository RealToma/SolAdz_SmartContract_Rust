use anchor_lang::prelude::*;

#[account]
pub struct AppStats {
  pub investor_count: u64,
  pub top_sponser_pool: u64,
  pub whale_pool: u64,
  pub total_deposits: u64,
  pub last_top_sponser_distribution: i64,
  pub owner: Pubkey,
  pub total_withdraw: u64,
  pub whale_counts: u64,
}