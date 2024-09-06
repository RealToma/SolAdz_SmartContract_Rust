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
  ctx.accounts.investor_account.current_cycle += 1;
  Ok(())
}