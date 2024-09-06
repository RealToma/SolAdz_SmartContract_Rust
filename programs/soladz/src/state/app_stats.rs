use anchor_lang::prelude::*;

#[account]
pub struct AppStats {
  pub owner: Pubkey,
  pub investor_count: u64,
  pub top_sponser_pool: u64,
  pub whale_pool: u64,
  pub total_deposits: u64,
  pub last_top_sponser_distribution: u64,
  pub last_whale_distribution: u64,
}