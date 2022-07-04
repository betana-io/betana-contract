use {
    anchor_lang::prelude::*,
    crate::{
        instruction::{InitArgs,StakePoolInstruction},
        error::StakePoolError
    },
    spl_stake_pool::{},
    borsh::{BorshDeserialize, BorshSerialize},
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo,
        decode_error::DecodeError,
        program_error::PrintProgramError,
        pubkey::Pubkey,
        clock::Clock,
        rent::Rent
    },
    spl_stake_pool::{
        instruction::{
            initialize
        }
    }
};

pub struct Processor;
impl Processor {

    fn process_setup_stake_pool(
        program_id: &Pubkey,
        init: InitArgs,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let stake_pool_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let validator_stake_list_info = next_account_info(account_info_iter)?;
        let pool_mint_info = next_account_info(account_info_iter)?;
        let owner_fee_info = next_account_info(account_info_iter)?;
        // Clock sysvar account
        let clock_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_info)?;
        // Rent sysvar account
        let rent_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_info)?;
        // Token program id
        let token_program_id = next_account_info(account_info_iter)?;

        // Check if tx was signed by owner
        if !owner_info.is_signer {
            return Err(StakePoolError::SignatureMissing.into());
        }

        let mut stake_pool = StakePoolInstruction::deserialize(&stake_pool_info.data.borrow())?;
        /*if stake_pool.is_initialized() {
            return Err(StakePoolError::AlreadyInUse.into());
        }*/

        
        // call add_validator_to_pool() from solana_program

        // emit StakePool event

        Ok(())
    }

    fn process_create_validator_stake_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        
        Ok(())
    }

    fn process_add_validator_stake_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        
        Ok(())
    }

    /// Bet added in the common match pool
    fn deposit_bet(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        // call deposit_stake() (for stake account) or deposit_stake_with_authority() (for private pool) or deposit_sol() or deposit_sol_with_authority() from solana_program

        // emit DepositBet event

        Ok(())
    }

    /// Calculate the different rewards for a match 
    fn calculate_rewards(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {

        Ok(())
    }

    /// Claim reward at the end of the match
    fn claim_rewards(
        program_id: &Pubkey,
        pool_amount: u64,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        // call withdraw_sol() from solana_program

        // emit RewardsWithdraw event

        Ok(())
    }

    /// Show the amount of the total pool
    fn get_stake_pool_balance(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {

        Ok(())
    }

    /// Processes [Instruction]
    ///
    /// 0. `[]` Program id of the currently executing program
    /// 1. `[]` Account because Solana programs are stateless
    /// 2. `[]` Data passed to the program by the caller, it could be anything
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        // Deserialize this instance from a slice of bytes.
        // Unpacks a byte buffer into a [StakePoolInstruction](enum.StakePoolInstruction.html).
        let instruction = StakePoolInstruction::deserialize(instruction_data)?;
        match instruction {
            StakePoolInstruction::Initialize(init) => {
                msg!("Instruction: Initialize");
                Self::process_setup_stake_pool(program_id, init, accounts)
            }
            StakePoolInstruction::DepositBet => {
                msg!("Instruction: DepositBet");
                Self::deposit_bet(program_id, accounts)
            }
            StakePoolInstruction::ClaimRewards(amount) => {
                msg!("Instruction: ClaimRewards");
                Self::claim_rewards(program_id, amount, accounts)
            }
            StakePoolInstruction::CreateValidatorStakeAccount => {
                msg!("Instruction: CreateValidatorStakeAccount");
                Self::process_create_validator_stake_account(program_id, accounts)
            }
            StakePoolInstruction::AddValidatorStakeAccount => {
                msg!("Instruction: DepositBet");
                Self::process_add_validator_stake_account(program_id, accounts)
            }
            StakePoolInstruction::CalculateRewards => {
                msg!("Instruction: CalculateRewards");
                Self::calculate_rewards(program_id, accounts)
            }
            StakePoolInstruction::GetPoolBalance => {
                msg!("Instruction: GetPoolBalance");
                Self::get_stake_pool_balance(program_id, accounts)
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
            StakePoolError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used."),
            StakePoolError::SignatureMissing => msg!("Error: Required signature is missing."),
            StakePoolError::PoolStateClose => msg!("Error: It's too late to add your bet in this pool because the game will start soon."),
            StakePoolError::PoolNotExist => msg!("Error: Invalide pool address."),
            StakePoolError::Deposit => msg!("Error: You don't have enough money in your wallet."),
            StakePoolError::InvalidClaim => msg!("Error: You canno't claim rewards because you didn't pick the correct team."),
            StakePoolError::CalculateRewardFailed => msg!("Error: The reward's calcul has failed."),
        }
    }
}