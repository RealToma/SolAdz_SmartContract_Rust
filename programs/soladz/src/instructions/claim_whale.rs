use anchor_lang::{
    prelude::*,
    solana_program::native_token::sol_to_lamports,
    system_program::{ transfer, Transfer },
};

use crate::{
    error::ErrorCode,
    AppStats,
    Investor,
    APP_STATS_SEED,
    INVESTOR_SEED,
    PERIOD,
    VAULT_SEED,
    WHALE_POOL_SEED,
};

#[derive(Accounts)]
pub struct ClaimWhale<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(
      mut,
      seeds = [INVESTOR_SEED, investor.key().as_ref()],
      bump
    )]
    pub investor_account: Box<Account<'info, Investor>>,

    #[account(
      mut,
      seeds = [WHALE_POOL_SEED],
      bump
    )]
    pub whale_pool: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [APP_STATS_SEED],
      bump
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimWhale<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.whale_pool.to_account_info().clone(),
            to: self.investor.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn claim_whale_handler(ctx: Context<ClaimWhale>) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if ctx.accounts.investor_account.last_update_whale + PERIOD < now {
        return err!(ErrorCode::InvalidTime);
    }
    if ctx.accounts.investor_account.amount < sol_to_lamports(2000_f64) {
        return err!(ErrorCode::InvalidWhale);
    }
    let amount: u64 = ctx.accounts.app_stats.whale_pool / ctx.accounts.app_stats.whale_counts;
    let bump: &[u8; 1] = &[ctx.bumps.whale_pool];
    let seeds: &[&[u8]] = &[VAULT_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    let lamports: u64 = if ctx.accounts.whale_pool.lamports() < amount {
        ctx.accounts.whale_pool.lamports()
    } else {
        amount
    };
    transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), lamports)?;
    if ctx.accounts.whale_pool.lamports() <= amount / 10 {
      ctx.accounts.app_stats.whale_pool = 0;
    }
    Ok(())
}
