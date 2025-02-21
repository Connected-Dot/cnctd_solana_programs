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

pub fn handler(ctx: Context<InitializeTreasury>, initial_admins: Option<Vec<Pubkey>>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;

    let admins = match initial_admins {
        Some(admins) if !admins.is_empty() => admins, // Use provided admins if not empty
        _ => vec![ctx.accounts.admin.key()], // Default to signer if `None` or empty
    };

    treasury.admins = admins;
    treasury.bump = ctx.bumps.treasury;
    treasury.version = 1;

    msg!("Treasury initialized with admins: {:?}", treasury.admins);
    
    Ok(())
}
