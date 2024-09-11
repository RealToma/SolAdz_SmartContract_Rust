use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Can not reach new cycle")]
    CantReachNewCycle,
    #[msg("Reached max level")]
    ReachedMaxLevel,
    #[msg("Invalid fee Account")]
    InvalidFeeAccount,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Arithmatic overflow")]
    ArithmaticOverflow
}
