use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::errors::CnctdStudioError;
use crate::state::treasury::Treasury;

#[derive(Accounts)]
pub struct CloseTreasury<'info> {
    #[account(
        mut,
        close = payer, // Closes account & refunds rent
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub payer: Signer<'info>, // The caller (receiver of funds)

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn close_treasury(ctx: Context<CloseTreasury>) -> Result<()> {
    if !ctx.accounts.treasury.is_admin(&ctx.accounts.payer.key()) {
        return Err(CnctdStudioError::Unauthorized.into());
    }
    
    let treasury = &ctx.accounts.treasury;
    let payer = &ctx.accounts.payer;

    require!(
        treasury.admins.contains(&payer.key()),
        CnctdStudioError::Unauthorized
    );

    msg!("Closing Treasury Account...");
    msg!("Bump: {}", treasury.bump);
    msg!("Version: {}", treasury.version);

    // Transfer remaining SOL in the PDA back to the payer
    let treasury_lamports = **ctx.accounts.treasury.to_account_info().lamports.borrow();
    **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() = 0;
    **ctx.accounts.payer.to_account_info().lamports.borrow_mut() += treasury_lamports;

    msg!("Transferred {} lamports to payer: {}", treasury_lamports, payer.key());

    Ok(())
}
