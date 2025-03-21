use anchor_lang::prelude::*;
use crate::{errors::CnctdStudioError, state::treasury::Treasury};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AdminAction {
    Add,
    Remove,
}

#[derive(Accounts)]
pub struct UpdateAdmins<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// The current admin performing the update (must be a signer)
    #[account(constraint = treasury.is_admin(&current_admin.key()))]
    pub current_admin: Signer<'info>,
    
    /// The admin being added or removed (doesn't need to sign)
    /// CHECK: We're just using this as a pubkey reference
    pub target_admin: UncheckedAccount<'info>,
}

pub fn update_admins(ctx: Context<UpdateAdmins>, action: AdminAction) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let target_key = ctx.accounts.target_admin.key();
    let admin = &ctx.accounts.current_admin;

    match action {
        AdminAction::Add => {
            // Only add if not already an admin
            if !treasury.is_admin(&target_key) {
                treasury.admins.push(target_key);
                msg!("Added new admin: {}", target_key);
            } else {
                msg!("Admin already exists: {}", target_key);

                return Err(CnctdStudioError::AdminAlreadyExists.into());
            }
        },
        AdminAction::Remove => {
            // Prevent removing the last admin
            if treasury.admins.len() <= 1 {
                return Err(error!(CnctdStudioError::CannotRemoveLastAdmin));
            }
            
            // Find and remove the admin
            let position = treasury.admins.iter().position(|x| x == &target_key);
            if let Some(index) = position {
                treasury.admins.remove(index);
                msg!("Removed admin: {}", target_key);
            } else {
                msg!("Admin not found: {}", target_key);

                return Err(CnctdStudioError::AdminNotFound.into());
            }
        }
    }
    
    Ok(())
}
