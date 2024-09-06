pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5LonPY5Qzt8c9qaSLtJhF5hxvgyPveKNs4eUgrYbuAjh");

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
}
