use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("CtgKMx2jcCWJ69TCn9octFYVv8XzUCkTpxFxNZuVKTN6");
#[program]
pub mod token_presale {
    use super::*;

    pub fn create_presale(
        ctx: Context<CreatePresale>,
        token_mint_address: Pubkey,
        quote_token_mint_address: Pubkey,
        hardcap_amount:u64,
        max_token_amount_per_address: u64,
        price_per_token: u64,
        start_time: u64,
        end_time: u64
    ) -> Result<()> {
        return create_presale::create_presale(
            ctx,
            token_mint_address,
            quote_token_mint_address,
            hardcap_amount,
            max_token_amount_per_address,
            price_per_token,
            start_time,
            end_time
        );
    }

    pub fn update_presale(
        ctx: Context<UpdatePresale>,
        max_token_amount_per_address: u64,
        price_per_token: u64,
        hardcap_amount: u64,
        start_time: u64,
        end_time: u64
    ) -> Result<()> {
        return update_presale::update_presale(
            ctx,
            max_token_amount_per_address,
            price_per_token,
            hardcap_amount,
            start_time,
            end_time
        );
    }

    pub fn deposit_token(
        ctx: Context<DepositToken>,
        amount: u64
    ) -> Result<()> {
        return deposit_token::deposit_token(
            ctx,
            amount
        );
    }

    pub fn start_presale(
        ctx: Context<StartPresale>,
        start_time: u64
    ) -> Result<()> {
        return start_presale::start_presale(
            ctx,
            start_time
        );
    }

    pub fn end_presale(
        ctx: Context<EndPresale>,
        end_time: u64
    ) -> Result<()> {
        return end_presale::end_presale(
            ctx,
            end_time
        );
    }

    pub fn buy_token(
        ctx: Context<BuyToken>,
        token_amount: u64,
        quote_amount: u64
    ) -> Result<()> {
        return buy_token::buy_token(
            ctx,
            token_amount,
            quote_amount
        );
    }

    pub fn claim_token(
        ctx: Context<ClaimToken>
    ) -> Result<()> {
        return claim_token::claim_token(
            ctx
        );
    }

    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        amount: u64
    ) -> Result<()> {
        return withdraw_token::withdraw_token(
            ctx,
            amount
        );
    }

    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        amount: u64
    ) -> Result<()> {
        return withdraw_sol::withdraw_sol(
            ctx,
            amount
        );
    }
}