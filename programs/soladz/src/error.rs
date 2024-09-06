use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Can not reach new cycle")]
    CantReachNewCycle,
}
