use anchor_lang::prelude::*;

// Not yet implemented

#[error_code]
pub enum PresaleError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
    #[msg("Presale not started yet")]
    PresaleNotStarted,
    #[msg("Insufficient Tokens")]
    InsufficientFund,
    #[msg("Wallet hardcap overflow")]
    WalletHardcapOverflow,
    #[msg("Global hardcap overflow")]
    GolbalHardcapOverflow,
    #[msg("Presale not ended yet")]
    PresaleNotEnded
}