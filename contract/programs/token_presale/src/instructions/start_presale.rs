use anchor_lang::prelude::*;

use crate::state::PresaleInfo;
use crate::constants::PRESALE_SEED;

// Start the presale
pub fn start_presale(
    ctx: Context<StartPresale>,
    start_time: u64
) -> Result<()> {
    
    let presale = &mut ctx.accounts.presale_info;

    presale.start_time = start_time;
    presale.is_live = true;

    msg!(
        "Presale has started for token: {}",
        presale.token_mint_address
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    start_time: u64
)]
pub struct StartPresale<'info> {
    
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