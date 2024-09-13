pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Bk8UyhwyA1WwbWzKtrb3cZV4rbfQuutwhXTAPWTPxHSU");

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

    pub fn init_investor_with_ref(ctx: Context<InitInvestorWithRef>, lamports: u64) -> Result<()> {
        init_investor_with_ref_handler(ctx, lamports)
    }

    pub fn matching_bonus_view (ctx: Context<MatchingBonusView>) -> Result<u64> {
        matching_bonus_view_handler(ctx)
    }

    pub fn run_distribution (ctx: Context<RunDistribution>, lamports: u64) -> Result<()> {
        run_distribution_handler(ctx, lamports)
    }

    pub fn claim_whale (ctx: Context<ClaimWhale>) -> Result<()> {
        claim_whale_handler(ctx)
    }
}
