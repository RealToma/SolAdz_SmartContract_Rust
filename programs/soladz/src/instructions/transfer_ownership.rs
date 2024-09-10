use anchor_lang::prelude::*;

use crate::{AppStats, APP_STATS_SEED};

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      mut,
      seeds = [APP_STATS_SEED],
      bump,
      has_one = owner,
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub new_owner: SystemAccount<'info>,
}


pub fn transfer_ownership_handler(ctx: Context<TransferOwnership>) -> Result<()> {
  ctx.accounts.app_stats.owner = ctx.accounts.new_owner.key();
  Ok(())
}