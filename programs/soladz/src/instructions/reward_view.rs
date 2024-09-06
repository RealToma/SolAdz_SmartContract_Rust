use anchor_lang::prelude::*;

use crate::Investor;

#[derive(Accounts)]
pub struct RewardView <'info> {
  pub investor_account: Box<Account<'info, Investor>>,
}

pub fn reward_view_handler(ctx: Context<RewardView>) -> Result<u64> {
  let reward: u64 = ctx.accounts.investor_account.calculate_reward();
  Ok(reward)
}