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

    /// Processes [Instruction]
    ///
    /// 0. `[]` Program id of the currently executing program
    /// 1. `[]` Account because Solana programs are stateless
    /// 2. `[]` Data passed to the program by the caller, it could be anything
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: input: &[u8]) -> ProgramResult {
        // Deserialize this instance from a slice of bytes.
        // Unpacks a byte buffer into a [StakePoolInstruction](enum.StakePoolInstruction.html).
        let instruction = StakePoolInstruction::try_from_slice(input)?;
        match instruction {
            StakePoolInstruction::InitializePool => {
                msg!("Instruction: InitializePool");
                Self::create_stake_pool()
            }
            StakePoolInstruction::DepositBet => {
                msg!("Instruction: DepositBet");
                Self::deposit_bet()
            }
            StakePoolInstruction::CalculateRewards => {
                msg!("Instruction: CalculateRewards");
                Self::withdraw_rewards()
            }
            StakePoolInstruction::GetPoolBalance => {
                msg!("Instruction: GetPoolBalance");
                Self::get_stake_pool_balance()
            }
        }
    }

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