use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Lottery is not open")]
    LotteryNotOpen,
}
