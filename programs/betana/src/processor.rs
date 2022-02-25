use {
    anchor_lang::prelude::*,
    crate::{
        instruction::StakePoolInstruction,
        error::StakePoolError
    },
    spl_stake_pool::{
        instruction::{initialize}
    },
    borsh::{BorshDeserialize, BorshSerialize},
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo,
        decode_error::DecodeError,
        program_error::PrintProgramError,
        pubkey::Pubkey
    }
};

pub struct Processor;
impl Processor {


}

impl PrintProgramError for StakePoolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            StakePoolError::PoolStateClose => msg!("Error: It's too late to add your bet in this pool because the game will start soon."),
            StakePoolError::PoolNotExist => msg!("Error: Invalide pool address."),
            StakePoolError::Deposit => msg!("Error: You don't have enough money in your wallet."),
            StakePoolError::InvalidClaim => msg!("Error: You canno't claim rewards because you didn't pick the correct team."),
            StakePoolError::CalculateRewardFailed => msg!("Error: The reward's calcul has failed."),
        }
    }
}