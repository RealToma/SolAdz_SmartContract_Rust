use anchor_lang::{ prelude::*, system_program::{transfer, Transfer} };

use crate::{ AppStats, APP_STATS_SEED, SEVER_SIGNER, VAULT_SEED };

#[derive(Accounts)]
pub struct RunDistribution<'info> {
    #[account(
      mut,
      address = SEVER_SIGNER
    )]
    pub signer: Signer<'info>,

    #[account(mut)]
    receiver: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [VAULT_SEED],
      bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [APP_STATS_SEED],
      bump
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub system_program: Program<'info, System>,
}

impl<'info> RunDistribution<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.vault.to_account_info().clone(),
            to: self.receiver.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn run_distribution_handler(ctx: Context<RunDistribution>, lamports: u64) -> Result<()> {
    let bump: &[u8; 1] = &[ctx.bumps.vault];
    let seeds: &[&[u8]] = &[VAULT_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), lamports)?;
    ctx.accounts.app_stats.top_sponser_pool -= lamports;
    Ok(())
}
