use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::PresaleInfo;
use crate::state::UserInfo;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;

pub fn buy_token(
    ctx: Context<BuyToken>, 
    token_amount: u64,
    quote_amount: u64
) -> Result<()> {
    // Immutable borrow of presale_info to check presale status
    if ctx.accounts.presale_info.is_live == false {
        msg!("Presale not started yet.");
        return Err(PresaleError::PresaleNotStarted.into());
    }

    if token_amount > ctx.accounts.presale_info.deposit_token_amount - ctx.accounts.presale_info.sold_token_amount {
        msg!("Insufficient tokens in presale");
        return Err(PresaleError::InsufficientFund.into());
    }

    if ctx.accounts.presale_info.max_token_amount_per_address < (ctx.accounts.user_info.buy_token_amount + token_amount) {
        msg!("Bought token amount exceeds wallet hardcap");
        return Err(PresaleError::WalletHardcapOverflow.into());
    }

    if ctx.accounts.presale_info.hardcap_amount < (ctx.accounts.presale_info.sold_token_amount + token_amount) {
        msg!("Total bought token amount exceeds global hardcap");
        return Err(PresaleError::GolbalHardcapOverflow.into());
    }
    
    // Immutable borrow of presale_info for the transfer
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.presale_info.to_account_info(), // Immutable borrow here
            })
        , quote_amount
    )?;

    msg!("Quote tokens transferred successfully.");

    // Mutable borrow of presale_info and user_info to update state
    let presale_info = &mut ctx.accounts.presale_info;
    let user_info = &mut ctx.accounts.user_info;

    user_info.buy_token_amount += token_amount;
    presale_info.sold_token_amount += token_amount;

    Ok(())
}


#[derive(Accounts)]
#[instruction(    
    token_amount: u64,
    quote_amount: u64
)]
pub struct BuyToken<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, presale_authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,
    pub presale_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + std::mem::size_of::<UserInfo>(),
        seeds = [USER_SEED, presale_authority.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}