use anchor_lang::{ prelude::*, system_program::{transfer, Transfer} };

use crate::{ Investor, INVESTOR_SEED, VAULT_SEED };

#[derive(Accounts)]
pub struct Claim<'info> {
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
      seeds = [VAULT_SEED],
      bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.vault.to_account_info().clone(),
            to: self.investor.to_account_info().clone(),
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn claim_handler(ctx: Context<Claim>) -> Result<()> {
    let bump: &[u8; 1] = &[ctx.bumps.vault];
    let seeds: &[&[u8]] = &[VAULT_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    let lamports: u64 = ctx.accounts.investor_account.calculate_reward();
    transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), lamports)?;
    let investor_account: &mut Box<Account<'_, Investor>> = &mut ctx.accounts.investor_account;
    investor_account.total_earned += lamports;
    investor_account.last_update = Clock::get().unwrap().unix_timestamp;
    Ok(())
}
