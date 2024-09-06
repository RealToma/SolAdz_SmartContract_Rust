use anchor_lang::prelude::*;

use std::mem::size_of;
use crate::{AppStats, APP_STATS_SEED};

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = size_of::<AppStats>() + 8,
        seeds = [APP_STATS_SEED],
        bump
    )]
    pub app_stats: Box<Account<'info, AppStats>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.owner = ctx.accounts.signer.key();
    Ok(())
}