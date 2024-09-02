use anchor_lang::prelude::*;

declare_id!("EX7SaCEZcWqM8reppz6tXqXsnczP2MFXPBaaXYAYwLYR");

#[program]
pub mod sol_adz {
    use super::*;

    // Initialization of investor and pools
    pub fn initialize(ctx: Context<Initialize>, investor_id: String) -> Result<()> {
        let investor = &mut ctx.accounts.investor;
        let pools = &mut ctx.accounts.pools;

        investor.investor_id = investor_id;
        investor.total_investment = 0;
        investor.daily_bonus = 0;
        investor.referral_earnings = 0;
        investor.matching_bonus = 0;
        investor.current_cycle = 0;
        investor.investment_this_cycle = 0;
        investor.cycle_cap = 100;  // Assuming the initial cap is 100 SOL
        investor.rank = "Starter".to_string();

        pools.top_sponsor_pool = 0;
        pools.whale_pool = 0;
        pools.admin_fee_collected = 0;

        Ok(())
    }

    // Apply daily bonus based on investment
    pub fn daily_bonus(ctx: Context<ApplyBonus>, investment: u64) -> Result<()> {
        let investor = &mut ctx.accounts.investor;
        investor.daily_bonus += investment / 100;  // Calculate 1% of the current investment
        Ok(())
    }

    // Handle direct referral commissions
    pub fn direct_referral_commission(ctx: Context<ReferralCommission>, amount_invested: u64, level: u8) -> Result<()> {
        let investor = &mut ctx.accounts.investor;
        let commission_rate = match level {
            1 => 10,
            2 => 2,
            _ => 1,
        };
        investor.referral_earnings += amount_invested * commission_rate / 100;
        Ok(())
    }

    // Handle daily matching bonuses
    pub fn daily_matching_bonus(ctx: Context<ApplyBonus>, generation: u8, daily_earnings: u64) -> Result<()> {
        let investor = &mut ctx.accounts.investor;
        let percentage = match generation {
            1 => 30,
            2..=5 => 10,
            6..=10 => 8,
            11..=15 => 5,
            16..=20 => 1,
            _ => 0,
        };
        let bonus = daily_earnings * percentage / 100;
        investor.matching_bonus += bonus;
        Ok(())
    }

    // Function to contribute to pools and apply an administrative fee
    pub fn manage_pools(ctx: Context<ManagePools>, deposit_amount: u64) -> Result<()> {
        let pools = &mut ctx.accounts.pools;
        pools.top_sponsor_pool += deposit_amount * 5 / 100;  // 5% to the top sponsor pool
        pools.whale_pool += deposit_amount * 25 / 1000;     // 2.5% to the whale pool
        pools.admin_fee_collected += deposit_amount * 5 / 100;  // 5% admin fee

        Ok(())
    }

    // New investment handling including cycle management
    pub fn new_investment(ctx: Context<Investment>, amount: u64) -> Result<()> {
        let investor = &mut ctx.accounts.investor;
        require!(amount >= investor.cycle_cap, ErrorCode::InvestmentBelowMinimum);

        investor.total_investment += amount;
        investor.investment_this_cycle += amount;

        if investor.investment_this_cycle >= investor.cycle_cap {
            // Handle cycle completion and prepare for next cycle
            investor.current_cycle += 1;
            investor.investment_this_cycle = 0;
            investor.cycle_cap *= 2;  // Double the cap for the next cycle
        }

        update_rank(investor);

        Ok(())
    }

    // manage admin account
    pub fn add_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        require!(admin_account.admins.iter().any(|x| *x == *ctx.accounts.admin.key), CustomError::Unauthorized);

        admin_account.admins.push(new_admin);
        Ok(())
    }

    pub fn remove_admin(ctx: Context<AdminAction>, admin_to_remove: Pubkey) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        require!(admin_account.admins.iter().any(|x| *x == *ctx.accounts.admin.key), CustomError::Unauthorized);

        admin_account.admins.retain(|&x| x != admin_to_remove);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 400)]
    pub investor: Account<'info, Investor>,
    #[account(init, payer = user, space = 8 + 48)]
    pub pools: Account<'info, Pools>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Investor {
    pub investor_id: String,
    pub total_investment: u64,
    pub daily_bonus: u64,
    pub referral_earnings: u64,
    pub matching_bonus: u64,
    pub current_cycle: u8,
    pub investment_this_cycle: u64,
    pub cycle_cap: u64,
    pub rank: String,
}

#[account]
pub struct Pools {
    pub top_sponsor_pool: u64,
    pub whale_pool: u64,
    pub admin_fee_collected: u64,
}

#[derive(Accounts)]
pub struct ApplyBonus<'info> {
    #[account(mut)]
    pub investor: Account<'info, Investor>,
}

#[derive(Accounts)]
pub struct ReferralCommission<'info> {
    #[account(mut)]
    pub investor: Account<'info, Investor>,
}

#[derive(Accounts)]
pub struct ManagePools<'info> {
    #[account(mut)]
    pub pools: Account<'info, Pools>,
}

#[derive(Accounts)]
pub struct Investment<'info> {
    #[account(mut)]
    pub investor: Account<'info, Investor>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, AdminAccount>,
    /// CHECK: This is checked in the program.
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminAccount {
    pub admins: Vec<Pubkey>, // You might want to limit the size or manage through a different strategy.
}

#[error_code]
pub enum ErrorCode {
    #[msg("The investment amount is below the minimum required for the current cycle.")]
    InvestmentBelowMinimum,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}

// Helper function to update investor rank based on total investment
fn update_rank(investor: &mut Account<Investor>) {
    investor.rank = match investor.total_investment {
        0..=100 => "Starter",     // Adjusted to integer range
        101..=990 => "Shrimp",
        1000..=9900 => "Crab",
        10000..=19900 => "Octopus",
        20000..=29900 => "Fish",
        30000..=49900 => "Dolphin",
        50000..=99900 => "Shark",
        _ => "Whale",
    }.to_string();
}