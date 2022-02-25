use {
    anchor_lang::prelude::*,
    crate::{
        instruction::StakePoolInstruction,
        error::StakePoolError
    },
    spl_stake_pool::{
        instruction::{initialize}
    },
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

    /**
     * 
     * createPool
     * 
     * create 3 pools - Pool A (team 1) / Pool Draw / Pool B (team 2)
     * 
     * @params match_id -> identifier of the match
     * 
     */
    pub fn create_stake_pool() {
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

    /**
     * 
     * depositPool
     * 
     * bet added in the common match pool  
     * 
     * @params bet -> amount that you want to bet for the match
     * @params match_id -> identifier of the match
     * @params pick -> which team will win the match 
     * 
     */
    pub fn deposit_bet() {
        // call deposit_stake() (for stake account) or deposit_stake_with_authority() (for private pool) or deposit_sol() or deposit_sol_with_authority() from solana_program

        // emit DepositBet event

        Ok(())
    }

    /**
     * 
     * calculateReward
     * 
     * calculate the different rewards for a match 
     * 
     * @params pool_address -> pool address relative to a match
     * @params user_address -> address of the sender
     * 
     */
    pub fn calculate_rewards() {

        Ok(())
    }

    /**
     * 
     * claimRewards
     * 
     * claim reward at the end of the match
     * 
     */
    pub fn withdraw_rewards() {
        // call withdraw_sol() from solana_program

        // emit RewardsWithdraw event

        Ok(())
    }

    /**
     * 
     * getPoolBalance
     * 
     * show the amount of the total pool
     * 
     * @params pool_address -> pool address relative to a match
     * 
     */
    pub fn get_stake_pool_balance() {

        Ok(())
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