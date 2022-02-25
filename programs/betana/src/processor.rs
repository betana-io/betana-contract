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

    /// Create 3 pools - Pool A (team 1) / Pool Draw / Pool B (team 2)
    pub fn create_stake_pool() -> ProgramResult {
        ///   Initializes a new StakePool.
        ///
        ///   0. `[w]` New StakePool to create.
        ///   1. `[s]` Manager
        ///   2. `[]` Staker
        ///   3. `[]` Stake pool withdraw authority
        ///   4. `[w]` Uninitialized validator stake list storage account
        ///   5. `[]` Reserve stake account must be initialized, have zero balance,
        ///       and staker / withdrawer authority set to pool withdraw authority.
        ///   6. `[]` Pool token mint. Must have zero supply, owned by withdraw authority.
        ///   7. `[]` Pool account to deposit the generated fee for manager.
        ///   8. `[]` Token program id
        ///   9. `[]` (Optional) Deposit authority that must sign all deposits.
        ///      Defaults to the program address generated using
        ///      `find_deposit_authority_program_address`, making deposits permissionless.

        // call initialize() from solana_program to Initialize a new StakePool
            
        // call add_validator_to_pool() from solana_program

        // emit StakePool event

        Ok(())
    }

    /// Bet added in the common match pool
    pub fn deposit_bet() -> ProgramResult {
        // call deposit_stake() (for stake account) or deposit_stake_with_authority() (for private pool) or deposit_sol() or deposit_sol_with_authority() from solana_program

        // emit DepositBet event

        Ok(())
    }

    /// Calculate the different rewards for a match 
    pub fn calculate_rewards() -> ProgramResult {

        Ok(())
    }

    /// Claim reward at the end of the match
    pub fn withdraw_rewards() -> ProgramResult {
        // call withdraw_sol() from solana_program

        // emit RewardsWithdraw event

        Ok(())
    }

    /// Show the amount of the total pool
    pub fn get_stake_pool_balance() -> ProgramResult {

        Ok(())
    }
    
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