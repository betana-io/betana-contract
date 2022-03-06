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
    /// Initializes a new StakePool, create 3 pools - Pool A (team 1) / Pool Draw / Pool B (team 2)
    ///
    /// 0. `[w]` New StakePool to create.
    /// 1. `[s]` Owner / Manager
    /// 2. `[w]` Uninitialized validator stake list storage account
    /// 3. `[]` Pool token Mint. Must be non zero, owned by withdraw authority.
    /// 4. `[]` Pool account to deposit the generated fee for owner / manager.
    /// 5. `[]` Token program id
    /// 6. `[]` Identifier of the match
    Initialize(InitArgs),

    /// Add a bet in the common match pool.
    ///
    /// 0. `[w]` Stake pool
    /// 1. `[w]` Validator stake list storage account
    /// 2. `[w]` Validator stake account for the stake account to be merged with
    /// 3. `[w]` Reserve stake account, to withdraw rent exempt reserve
    /// 4. `[w]` User account to receive pool tokens
    /// 5. `[w]` Account to receive pool fee tokens
    /// 6. `[w]` Account to receive a portion of pool fee tokens as referral fees
    /// 7. `[w]` Pool token mint account
    /// 8. `[]` Pool token program id,
    /// 9. `[]` Amount that you want to bet for the match
    /// 10. `[]` Identifier of the match
    /// 11. `[]` Team that will win the match
    DepositBet,

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
            1 => Self::DepositBet,
            2 => Self::CalculateRewards,
            3 => Self::GetPoolBalance,
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
            Self::DepositBet => {
                output[0] = 1;
            },
            Self::CalculateRewards => {
                output[0] = 2;
            },
            Self::GetPoolBalance => {
                output[0] = 3;
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
        AccountMeta::new_readonly(*token_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn deposit_bet(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    validator_stake_list: &Pubkey,
    stake_pool_deposit: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    stake_account_join: &Pubkey,
    validator_stake_account: &Pubkey,
    reserve_stake_account: &Pubkey,
    pool_tokens_to: &Pubkey,
    manager_fee_account: &Pubkey,
    referrer_pool_tokens_account: &Pubkey,
    pool_mint: &Pubkey,
    token_program_id: &Pubkey,
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
        AccountMeta::new(*stake_account_join, false),
        AccountMeta::new(*validator_stake_account, false),
        AccountMeta::new(*reserve_stake_account, false),
        AccountMeta::new(*pool_tokens_to, false),
        AccountMeta::new(*manager_fee_account, false),
        AccountMeta::new(*referrer_pool_tokens_account, false),
        AccountMeta::new(*pool_mint, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}