use anchor_lang::prelude::*;

use crate::state::PresaleInfo;
use crate::constants::PRESALE_SEED;

// End the presale
pub fn end_presale(
    ctx: Context<EndPresale>,
    end_time: u64
) -> Result<()> {
    
    let presale = &mut ctx.accounts.presale_info;

    presale.end_time = end_time;
    presale.is_live = false;

    msg!(
        "Presale has ended for token: {}",
        presale.token_mint_address
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    end_time: u64
)]
pub struct EndPresale<'info> {
    
    #[account(
        mut,
        seeds = [PRESALE_SEED, authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,
    
    // Set the authority to the transaction signer
    #[account(
        mut,
        constraint = authority.key() == presale_info.authority
    )]
    pub authority: Signer<'info>,
}