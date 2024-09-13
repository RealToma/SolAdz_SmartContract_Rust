use anchor_lang::prelude::*;

#[account]
pub struct Investor {
  pub investor: Pubkey,
  pub amount: u64,
  pub last_update: i64,
  pub current_cycle: u8,
  pub total_earned: u64,
  pub cycle_completed: bool,
  pub referred_count: u16,
  pub referred_amount: u64,
  pub last_update_commission: i64,
  pub last_update_whale: i64,
}

impl Investor {
  pub fn calculate_reward(&self) -> u64 {
    let percentage: u64 = self.get_reward_percentage();
    let reward: u64 = self.amount * percentage;
    return reward;
  }

  pub fn calculate_matching_bonus(&self) -> u64 {
    let mut bonus: u64 = 0;
    if self.referred_count == 1 {
      bonus = self.referred_amount / 10000_u64 * 30_u64;
    }
    if self.referred_count >=2 && self.referred_count <=5 {
      bonus = self.referred_amount / 10000_u64 * 10_u64;
    }
    if self.referred_count >=6 && self.referred_count <=10 {
      bonus = self.referred_amount / 10000_u64 * 8_u64;
    }
    if self.referred_count >=11 && self.referred_count <=15 {
      bonus = self.referred_amount / 10000_u64 * 5_u64;
    }
    if self.referred_count >=16 && self.referred_count <=20 {
      bonus = self.referred_amount / 10000_u64;
    }
    return bonus;
  }

  pub fn get_reward_percentage(&self) -> u64 {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    let mut percentage: u64 = (now - self.last_update) as u64 / 86400_u64 / 100;
    if percentage > 300 {
      percentage = 300;
    }
    return percentage;
  }
}