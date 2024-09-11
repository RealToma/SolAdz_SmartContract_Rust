pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2j9DGuVfCnVWwgHkhqerYwjym7SXTF7J96npgKd7We8i");

#[program]
pub mod soladz {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn invest (ctx: Context<Invest>, lamports: u64) -> Result<()> {
        invest_handler(ctx, lamports)
    }

    pub fn owner_withdraw (ctx: Context<OwnerWithdraw>) -> Result<()> {
        owner_withdraw_handler(ctx)
    }

    pub fn reward_view (ctx: Context<RewardView>) -> Result<u64> {
        reward_view_handler(ctx)
    }

    pub fn claim (ctx: Context<Claim>) -> Result<()> {
        claim_handler(ctx)
    }

    pub fn investor_withdraw(ctx: Context<InvestorWithdraw>, lamports: u64) -> Result<()> {
        investor_withdraw_handler(ctx, lamports)
    }

    pub fn open_new_cycle (ctx: Context<OpenNewCycle>) -> Result<()> {
        open_new_cycle_handler(ctx)
    }

    pub fn claim_direct_commision (
        ctx: Context<ClaimDirectCommision>,
        lamports: u64
    ) -> Result<()>  {
        claim_direct_commision_handler(ctx, lamports)
    }

    pub fn transfer_ownership (ctx: Context<TransferOwnership>) -> Result<()> {
        transfer_ownership_handler(ctx)
    }
}
