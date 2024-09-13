use anchor_lang::{
    prelude::*,
    solana_program::native_token::sol_to_lamports,
    system_program::{ transfer, Transfer },
};

use std::mem::size_of;
use crate::{
    error::ErrorCode,
    AppStats,
    Investor,
    ADMIN,
    APP_STATS_SEED,
    INVESTOR_SEED,
    VAULT_SEED,
    WHALE_POOL_SEED,
};

#[derive(Accounts)]
pub struct Invest<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(
        init_if_needed,
        payer = investor,
        space = size_of::<Investor>() + 8,
        seeds = [INVESTOR_SEED, investor.key().as_ref()],
        bump
    )]
    pub investor_account: Box<Account<'info, Investor>>,

    #[account(
      mut,
      seeds = [VAULT_SEED],
      bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [WHALE_POOL_SEED],
        bump
      )]
    pub whale_pool: SystemAccount<'info>,

    #[account(
      mut,
      address = ADMIN
    )]
    pub fee_account: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [APP_STATS_SEED],
      bump
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub system_program: Program<'info, System>,
}

impl<'info> Invest<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.investor.to_account_info().clone(),
            to: self.vault.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }

    fn transfer_fee_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.investor.to_account_info().clone(),
            to: self.fee_account.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }

    fn transfer_whale_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.investor.to_account_info().clone(),
            to: self.whale_pool.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn invest_handler(ctx: Context<Invest>, lamports: u64) -> Result<()> {
    if ctx.accounts.investor.lamports() < lamports {
        return err!(ErrorCode::InsufficientBalance);
    }
    if lamports < sol_to_lamports(0.1) || lamports > sol_to_lamports(1000_f64) {
        return err!(ErrorCode::InvalidAmount);
    }
    let admin_fee: u64 = (lamports * 5) / 100; // 5% admin fee
    let whale_fee: u64 = (lamports * 25) / 1000; // 2.5% to whale pool
    transfer(ctx.accounts.transfer_context(), lamports - admin_fee - whale_fee)?;
    transfer(ctx.accounts.transfer_whale_context(), whale_fee)?;
    transfer(ctx.accounts.transfer_fee_context(), admin_fee)?;
    let investor_account: &mut Box<Account<'_, Investor>> = &mut ctx.accounts.investor_account;
    investor_account.investor = ctx.accounts.investor.key();
    if investor_account.amount == 0 {
        ctx.accounts.app_stats.investor_count += 1;
        investor_account.amount = lamports - admin_fee;
        investor_account.current_cycle = 1;
    } else {
        investor_account.amount += lamports - admin_fee;
    }
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    investor_account.last_update = now;
    investor_account.last_update_commission = now;
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.total_deposits = app_stats.total_deposits.checked_add(lamports).unwrap();
    let top_sponsor_fee: u64 = (lamports * 5) / 100; // 5% to top sponsor pool
    app_stats.top_sponser_pool = app_stats.top_sponser_pool.checked_add(top_sponsor_fee).unwrap();
    app_stats.whale_pool = app_stats.whale_pool.checked_add(whale_fee).unwrap();
    if investor_account.amount >= sol_to_lamports(2000_f64) {
        app_stats.whale_counts += 1;
    }
    Ok(())
}
