use anchor_lang::prelude::*;

/// Emitted when a Stake Pool is created.
#[event]
pub struct StakePool {}

/// Emitted when a user has Deposit a Bet.
#[event]
pub struct DepositBet {}

/// Emitted when a Rewards has been Withdraw
#[event]
pub struct RewardsWithdraw {}