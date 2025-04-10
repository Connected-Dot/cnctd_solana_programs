use anchor_lang::prelude::*;

use crate::{arguments::release::CloseReleaseAccessArgs, state::{release_access::ReleaseAccess, treasury::Treasury }, utils::UuidFormatting};

#[derive(Accounts)]
#[instruction(args: CloseReleaseAccessArgs)]
pub struct CloseReleaseAccess<'info> {
    /// Admin that is authorized to close the account
    #[account(constraint = treasury.is_admin(&admin.key()))]
    pub admin: Signer<'info>,
        
    /// The treasury that will receive the reclaimed rent
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// The user PDA being closed
    #[account(
        mut,
        seeds = [
            b"release_access",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump,
        close = treasury // Send lamports back to treasury
    )]
    pub release_access: Account<'info, ReleaseAccess>,
    
    pub system_program: Program<'info, System>,
}

pub fn close_release_access(ctx: Context<CloseReleaseAccess>, args: CloseReleaseAccessArgs) -> Result<()> {
    // The account closing is handled automatically by Anchor via the `close = treasury` constraint
    msg!("Release Access account {} closed and rent returned to treasury {}", ctx.accounts.release_access.key(), ctx.accounts.treasury.key());

    ctx.accounts.treasury.reimburse_admin(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.admin.to_account_info(),
        args.fee_compensation,
        None,
    )?;
    
    Ok(())
}