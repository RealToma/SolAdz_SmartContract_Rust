use anchor_lang::prelude::*;

use crate::{AppStats, APP_STATS_SEED};

#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        has_one = owner,
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub new_owner: Signer<'info>
}

pub fn change_owner_handler(ctx: Context<ChangeOwner>) -> Result<()> {
  ctx.accounts.app_stats.owner = ctx.accounts.new_owner.key();
  Ok(())
}
