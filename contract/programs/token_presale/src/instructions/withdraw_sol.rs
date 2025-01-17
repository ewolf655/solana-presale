use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::PresaleInfo;
use crate::constants::PRESALE_SEED;

pub fn withdraw_sol(
    ctx: Context<WithdrawSol>, 
    amount: u64
) -> Result<()> {

    let bump = &[ctx.accounts.presale_info.bump];

    **ctx.accounts.presale_info.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.buyer.try_borrow_mut_lamports()? += amount;

    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.presale_info.to_account_info(),
                to: ctx.accounts.buyer.to_account_info(),
            },
            &[&[PRESALE_SEED, ctx.accounts.presale_authority.key().as_ref(), bump][..]],
        )
        ,amount
    )?;

    msg!("Withdraw sol successfully.");

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64
)]
pub struct WithdrawSol<'info> {

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
        constraint = buyer.key() == presale_info.authority
    )]
    pub buyer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}