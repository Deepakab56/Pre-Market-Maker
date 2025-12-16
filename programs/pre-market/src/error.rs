use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("already Exist your token")]
    AlreadyExists,
    #[msg("token is not register")]
    NotRegisterTokens,
    #[msg("invalid token Address")]
    InvalidTokenAddress,
    #[msg("invalid end Time")]
    InvalidEndTime,
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("invalid create Id")]
    InvalidCreateId,
    #[msg("Invalid Order Id")]
    InvalidOrderId,
    #[msg("Order is not active")]
    OrderInActive,
    #[msg("E22_INVALID_COLLATERAL_STATE")]
    InsufficentOrder,
}
