use anchor_lang::prelude::*;
use anchor_lang::account;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod betana {

    use anchor_lang::solana_program::{program::invoke,system_instruction::transfer};
    use super::*;

    pub fn place_bet(
        ctx: Context<SendSol>, 
        id_match: u64, 
        id_team: u64, 
        amount: u64, 
        user_address: Pubkey
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        //Build the struct.
        let id_match : u64 = id_match;
        let id_team : u64 = id_team;
        let amount : u64 = amount;

        let bet_item = BetStruct {
            id_match: id_match,
            id_team: id_team,
            amount: amount,
            user_address: *ctx.accounts.from.to_account_info().key,
        };

        base_account.current_bet = bet_item;

        let transfer_amount = amount as u64;

        if transfer_amount > 0 {
            let ix = &transfer(
                &ctx.accounts.from.key(),
                &ctx.accounts.to.key(),
                transfer_amount
            );
            invoke(
                &ix,
                &[
                    ctx.accounts.from.to_account_info(),
                    ctx.accounts.to.to_account_info()
                ],
                
            );
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BetStruct {
    pub id_match: u64,
    pub id_team: u64,
    pub amount: u64,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub current_bet: BetStruct,
}