use anchor_lang::prelude::*;

use crate::state::{treasury::Treasury, user_pda::UserPDA};

#[derive(Accounts)]
#[instruction(user_id: String)]
pub struct CloseUserAccount<'info> {
    /// The user PDA being closed
    #[account(
        mut,
        seeds = [b"user", user_id.as_bytes()],
        bump,
        close = treasury // Send lamports back to treasury
    )]
    pub user_pda: Account<'info, UserPDA>,
    
    /// The treasury that will receive the reclaimed rent
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// Admin that is authorized to close the account
    #[account(constraint = treasury.is_admin(&admin.key()))]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn close_user_account(ctx: Context<CloseUserAccount>, user_id: String) -> Result<()> {
    // The account closing is handled automatically by Anchor via the `close = treasury` constraint
    msg!("User account {} closed and rent returned to treasury {}",
         ctx.accounts.user_pda.key(),
         ctx.accounts.treasury.key());
    
    Ok(())
}