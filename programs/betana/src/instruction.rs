#![allow(clippy::too_many_arguments)]

use {
    std::mem::size_of,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar
    }
};

/// Fee is minted on deposit
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fee {
    /// denominator of the fee ratio
    pub denominator: u64,
    /// numerator of the fee ratio
    pub numerator: u64,
}

/// Inital values for the Stake Pool
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitArgs {
    /// withdraw and deposit fees can be add
    /// Fee paid to the owner in pool tokens
    pub fee: Fee,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StakePoolInstruction {
    /// Admin: Initializes a new StakePool, create 3 pools - Pool A (team 1) / Pool Draw / Pool B (team 2)
    ///
    /// 0. `[w]` New StakePool to create.
    /// 1. `[s]` Owner / Manager
    /// 2. `[w]` Uninitialized validator stake list storage account
    /// 3. `[]` Pool token Mint. Must be non zero, owned by withdraw authority.
    /// 4. `[]` Pool account to deposit the generated fee for owner / manager.
    /// 5. `[]` Clock sysvar
    /// 6. `[]` Rent sysvar
    /// 7. `[]` Token program id

    /// 8. `[]` Identifier of the match
    Initialize(InitArgs),

    ///   Admin: Creates new program account for accumulating stakes for a particular validator
    ///
    ///   0. `[]` Stake pool account this stake will belong to
    ///   1. `[ws]` Funding account (must be a system account)
    ///   2. `[w]` Stake account to be created
    ///   3. `[]` Validator this stake account will vote for
    ///   4. `[]` Stake authority for the new stake account
    ///   5. `[]` Withdraw authority for the new stake account
    ///   6. `[]` Rent sysvar
    ///   7. `[]` System program
    ///   8. `[]` Stake program
    CreateValidatorStakeAccount,

    ///   Admin: Adds validator stake account to the pool
    ///
    ///   0. `[w]` Stake pool
    ///   1. `[s]` Owner
    ///   2. `[]` Stake pool deposit authority
    ///   3. `[]` Stake pool withdraw authority
    ///   4. `[w]` Validator stake list storage account
    ///   5. `[w]` Stake account to add to the pool, its withdraw authority should be set to stake pool deposit
    ///   6. `[w]` User account to receive pool tokens
    ///   7. `[w]` Pool token mint account
    ///   8. `[]` Clock sysvar (required)
    ///   9. '[]' Sysvar stake history account
    ///  10. `[]` Pool token program id,
    ///  11. `[]` Stake program id,
    AddValidatorStakeAccount,

    /// Add a bet in the common match pool.
    ///
    /// 0. `[w]` Stake pool
    /// 1. `[w]` Validator stake list storage account
    /// 2. `[w]` Stake pool deposit authority
    /// 3. `[w]` Stake pool withdraw authority
    /// 4. `[w]` Stake account to join the pool
    /// 5. `[w]` Validator stake account for the stake account to be merged with
    /// 6. `[w]` User account to receive pool tokens
    /// 7. `[w]` Account to receive pool fee tokens
    /// 8. `[w]` Pool token mint account
    /// 9. '[]' Sysvar clock account (required)
    /// 10. '[]' Sysvar stake history account
    /// 11. `[]` Pool token program id,
    /// 12. `[]` Stake program id,

    /// 13. `[]` Amount that you want to bet for the match
    /// 14. `[]` Identifier of the match
    /// 15. `[]` Team that will win the match
    DepositBet,

    /// Withdraw the token from the pool at the current ratio.
    /// The amount withdrawn is the MIN(u64, stake size)
    ///
    /// 0. `[w]` Stake pool
    /// 1. `[w]` Validator stake list storage account
    /// 2. `[]` Stake pool withdraw authority
    /// 3. `[w]` Validator stake account to split
    /// 4. `[w]` Unitialized stake account to receive withdrawal
    /// 5. `[]` User account to set as a new withdraw authority
    /// 6. `[w]` User account with pool tokens to burn from
    /// 7. `[w]` Pool token mint account
    /// 8. `[]` Sysvar stake history account
    /// 9. `[]` Pool token program id
    /// 10. `[]` Stake program id
    /// userdata: amount to withdraw
    ClaimRewards(u64),

    /// Calculate the different rewards for a match 
    ///
    /// 0. `[]` Pool address relative to a match
    /// 1. `[]` Address of the sender
    CalculateRewards,

    /// Show the amount of the total pool
    ///
    /// 0. `[]` Pool address relative to a match
    GetPoolBalance
}

impl StakePoolInstruction {
    /// Deserializes a byte buffer into an [StakePoolInstruction](enum.StakePoolInstruction.html).
    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < size_of::<u8>() {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(match input[0] {
            0 => {
                let val: &InitArgs = unpack(input)?;
                Self::Initialize(*val)
            },
            1 => self::CreateValidatorStakeAccount,
            2 => self::AddValidatorStakeAccount,
            3 => Self::DepositBet,
            4 => Self::ClaimRewards,
            5 => Self::CalculateRewards,
            6 => Self::GetPoolBalance,
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }

    /// Serializes an [StakePoolInstruction](enum.StakePoolInstruction.html) into a byte buffer.
    pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
        let mut output = vec![0u8; size_of::<StakePoolInstruction>()];
        match self {
            Self::Initialize(init) => {
                output[0] = 0;
                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut InitArgs) };
                *value = *init;
            },

            Self::CreateValidatorStakeAccount => {
                output[0] = 1;
            },
            Self::AddValidatorStakeAccount => {
                output[0] = 2;
            }
            Self::DepositBet => {
                output[0] = 3;
            },
            Self::ClaimRewards => {
                output[0] = 4;
            },
            Self::CalculateRewards => {
                output[0] = 5;
            },
            Self::GetPoolBalance => {
                output[0] = 6;
            },
        }

        Ok(output)
    }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
    if input.len() < size_of::<u8>() + size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    #[allow(clippy::cast_ptr_alignment)]
    let val: &T = unsafe { &*(&input[1] as *const u8 as *const T) };
    Ok(val)
}

/// Creates an 'initialize' instruction.
pub fn initialize(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    manager: &Pubkey,
    validator_stake_list: &Pubkey,
    pool_mint: &Pubkey,
    manager_pool_account: &Pubkey,
    token_program_id: &Pubkey,
    init_args: InitArgs,
    // match_id
) -> Result<Instruction, ProgramError> {
    let init_data = StakePoolInstruction::Initialize(init_args);
    let data = init_data.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*manager, true),
        AccountMeta::new(*validator_stake_list, false),
        AccountMeta::new(*pool_mint, false),
        AccountMeta::new_readonly(*manager_pool_account, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates `CreateValidatorStakeAccount` instruction (create new stake account for the validator)
pub fn create_validator_stake_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    funder: &Pubkey,
    stake_account: &Pubkey,
    validator: &Pubkey,
    stake_authority: &Pubkey,
    withdraw_authority: &Pubkey,
    system_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new_readonly(*stake_pool, false),
        AccountMeta::new(*funder, true),
        AccountMeta::new(*stake_account, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(*stake_authority, false),
        AccountMeta::new_readonly(*withdraw_authority, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(*system_program_id, false),
        AccountMeta::new_readonly(*stake_program_id, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::CreateValidatorStakeAccount.serialize()?,
    })
}

/// Creates `AddValidatorStakeAccount` instruction (add new validator stake account to the pool)
pub fn add_validator_stake_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    owner: &Pubkey,
    stake_pool_deposit: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_account: &Pubkey,
    pool_tokens_to: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new_readonly(*stake_pool_deposit, false),
        AccountMeta::new_readonly(*stake_pool_withdraw, false),
        AccountMeta::new(*validator_stake_list, false),
        AccountMeta::new(*stake_account, false),
        AccountMeta::new(*pool_tokens_to, false),
        AccountMeta::new(*pool_mint, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new_readonly(*stake_program_id, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::AddValidatorStakeAccount.serialize()?,
    })
}

pub fn deposit_bet(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_pool_deposit: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_to_join: &Pubkey,
    validator_stake_account: &Pubkey,
    pool_tokens_to: &Pubkey,
    pool_fee_to: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
    // bet
    // match_id
    // pick
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::DepositBet;
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new(*validator_stake_list, false),
        AccountMeta::new_readonly(*stake_pool_deposit, false),
        AccountMeta::new_readonly(*stake_pool_withdraw, false),
        AccountMeta::new(*stake_to_join, false),
        AccountMeta::new(*validator_stake_account, false),
        AccountMeta::new(*pool_tokens_to, false),
        AccountMeta::new(*pool_fee_to, false),
        AccountMeta::new(*pool_mint, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new_readonly(*stake_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn claim_rewards(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_to_split: &Pubkey,
    stake_to_receive: &Pubkey,
    user_withdrawer: &Pubkey,
    burn_from: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
    stake_program_id: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::ClaimRewards(amount);
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new(*validator_stake_list, false),
        AccountMeta::new(*stake_pool_withdraw, false),
        AccountMeta::new(*stake_to_split, false),
        AccountMeta::new(*stake_to_receive, false),
        AccountMeta::new(*user_withdrawer, false),
        AccountMeta::new(*burn_from, false),
        AccountMeta::new(*pool_mint, false),
        AccountMeta::new(sysvar::clock::id(), false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new(*stake_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}