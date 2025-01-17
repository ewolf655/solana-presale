use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::PresaleInfo;
use crate::constants::PRESALE_SEED;
use crate::errors::PresaleError;

pub fn withdraw_token(
    ctx: Context<WithdrawToken>, 
    amount: u64
) -> Result<()> {
    // Validate token availability with an immutable borrow
    if ctx.accounts.presale_info.deposit_token_amount < amount {
        return Err(PresaleError::InsufficientFund.into());
    }

    msg!("Withdrawing presale tokens from the contract...");
    msg!("Mint: {}", &ctx.accounts.presale_token_mint_account.to_account_info().key());
    msg!("From Token Address: {}", &ctx.accounts.presale_presale_token_associated_token_account.key());
    msg!("To Token Address: {}", &ctx.accounts.buyer_presale_token_associated_token_account.key());

    let bump = &[ctx.accounts.presale_info.bump];
    let binding = ctx.accounts.presale_authority.key();
    let presale_seeds = &[
        PRESALE_SEED,
        binding.as_ref(),
        bump,
    ];

    // Perform the token transfer with an immutable borrow
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.presale_presale_token_associated_token_account.to_account_info(),
                to: ctx.accounts.buyer_presale_token_associated_token_account.to_account_info(),
                authority: ctx.accounts.presale_info.to_account_info(),
            },
            &[presale_seeds],
        ),
        amount,
    )?;

    msg!("Tokens withdrawn successfully.");

    // Mutably update the deposit_token_amount after the transfer
    let presale_info = &mut ctx.accounts.presale_info;
    presale_info.deposit_token_amount -= amount;

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64
)]
pub struct WithdrawToken<'info> {
    // Presale token accounts
    #[account(mut)]
    pub presale_token_mint_account: Box<Account<'info, token::Mint>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = buyer_authority,
    )]
    pub buyer_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = presale_info,
    )]
    pub presale_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        seeds = [PRESALE_SEED, presale_authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,
    pub presale_authority: SystemAccount<'info>,
    #[account(constraint = buyer.key() == buyer_authority.key())]
    pub buyer_authority: SystemAccount<'info>,
    #[account(
        mut,
        constraint = buyer.key() == presale_info.authority.key()
    )]
    pub buyer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}