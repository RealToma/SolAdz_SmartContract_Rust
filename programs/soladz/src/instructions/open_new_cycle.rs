use anchor_lang::prelude::*;

use crate::{ Investor, INVESTOR_SEED, error::ErrorCode };

#[derive(Accounts)]
pub struct OpenNewCycle<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(
      mut,
      seeds = [INVESTOR_SEED, investor.key().as_ref()],
      bump
    )]
    pub investor_account: Box<Account<'info, Investor>>,
}

pub fn open_new_cycle_handler(ctx: Context<OpenNewCycle>) -> Result<()> {
  let percentage: u64 = ctx.accounts.investor_account.get_reward_percentage();
  if percentage < 300 {
    return err!(ErrorCode::CantReachNewCycle);
  }
  if ctx.accounts.investor_account.current_cycle == 8 {
    return err!(ErrorCode::ReachedMaxLevel);
  }
  let investor_account: &mut Box<Account<'_, Investor>> = &mut ctx.accounts.investor_account;
  investor_account.current_cycle += 1;
  investor_account.amount *= 2;
  Ok(())
}