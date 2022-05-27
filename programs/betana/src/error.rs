use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakePoolError {
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,

    /// Required signature is missing
    #[error("SignatureMissing")]
    SignatureMissing,

    /// It's too late to add your bet in this pool because the game will start soon.
    #[error("PoolStateClose")]
    PoolStateClose,

    // Invalide pool address.
    #[error("PoolNotExist")]
    PoolNotExist,

    /// You don't have enough money in your wallet.
    #[error("Deposit")]
    Deposit,

    /// You canno't claim rewards because you didn't pick the correct team.
    #[error("InvalidClaim")]
    InvalidClaim,

    // The reward's calcul has failed.
    #[error("CalculateRewardFailed")]
    CalculateRewardFailed
}

/// Convert StakePoolError into a ProgramError
impl From<StakePoolError> for ProgramError {
    fn from(e: StakePoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}