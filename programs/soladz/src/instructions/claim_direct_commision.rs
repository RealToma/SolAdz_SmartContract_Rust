use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

use crate::{ AppStats, Investor, APP_STATS_SEED, INVESTOR_SEED, PERIOD, SEVER_SIGNER, VAULT_SEED, error::ErrorCode };

#[derive(Accounts)]
pub struct ClaimDirectCommision<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(address = SEVER_SIGNER)]
    pub server_signer: Signer<'info>,

    #[account(
      mut,
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
      seeds = [APP_STATS_SEED],
      bump
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimDirectCommision<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.vault.to_account_info().clone(),
            to: self.investor.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn claim_direct_commision_handler(
    ctx: Context<ClaimDirectCommision>,
    lamports: u64
) -> Result<()> {
  let now: i64 = Clock::get().unwrap().unix_timestamp;
    if now - ctx.accounts.investor_account.last_update_commission < PERIOD {
      return err!(ErrorCode::InvalidTime);
    }
    let bump: &[u8; 1] = &[ctx.bumps.vault];
    let seeds: &[&[u8]] = &[VAULT_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), lamports)?;
    ctx.accounts.app_stats.total_withdraw += lamports;
    ctx.accounts.investor_account.last_update_commission = now;
    Ok(())
}
