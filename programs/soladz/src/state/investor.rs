use anchor_lang::prelude::*;

#[account]
pub struct Investor {
  pub investor: Pubkey,
  pub is_admin: bool,
  pub amount: u64,
  pub last_update: i64,
  pub current_cycle: u8,
  pub total_earned: u64,
  pub cycle_completed: bool,
  pub referrer: Pubkey,
}

impl Investor {
  pub fn calculate_reward(&self) -> u64 {
    let percentage: u64 = self.get_reward_percentage();
    let reward: u64 = self.amount * percentage / 100_u64;
    return reward;
  }

  pub fn get_reward_percentage(&self) -> u64 {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    let mut percentage: u64 = (now - self.last_update) as u64 / 86400_u64;
    if percentage > 300 {
      percentage = 300;
    }
    return percentage;
  }
}