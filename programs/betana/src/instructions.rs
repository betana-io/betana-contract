pub enum StakePoolInstruction {
    /// Initializes a new StakePool, create 3 pools - Pool A (team 1) / Pool Draw / Pool B (team 2)
    ///
    /// 0. `[]` Identifier of the match
    InitializePool,

    /// Add a bet in the common match pool.
    ///
    /// 0. `[]` Amount that you want to bet for the match
    /// 1. `[]` Identifier of the match
    /// 2. `[]` Team that will win the match
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
