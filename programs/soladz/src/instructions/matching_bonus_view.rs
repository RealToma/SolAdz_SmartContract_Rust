use anchor_lang::prelude::*;

use crate::Investor;

#[derive(Accounts)]
pub struct MatchingBonusView <'info> {
  pub investor_account: Box<Account<'info, Investor>>,
}

pub fn matching_bonus_view_handler(ctx: Context<MatchingBonusView>) -> Result<u64> {
  let bonus = ctx.accounts.investor_account.calculate_matching_bonus();
  Ok(bonus)
}