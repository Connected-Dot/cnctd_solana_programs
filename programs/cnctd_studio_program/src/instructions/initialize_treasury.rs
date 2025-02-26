use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;

#[derive(Accounts)]
#[instruction(initial_admins: Vec<Pubkey>)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + (32 * 5) + 1, // Supports up to 5 admin pubkeys
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeTreasury>, mut initial_admins: Vec<Pubkey>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let payer_pubkey = ctx.accounts.admin.key();

    // If no admins are provided, default to the payer
    if initial_admins.is_empty() {
        initial_admins.push(payer_pubkey);
    }

    treasury.admins = initial_admins;
    treasury.bump = ctx.bumps.treasury;
    treasury.version = 1;

    msg!("Treasury initialized with admins: {:?}", treasury.admins);
    
    Ok(())
}

