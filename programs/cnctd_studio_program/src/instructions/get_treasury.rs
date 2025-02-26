use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;

#[derive(Accounts)]
pub struct GetTreasury<'info> {
    #[account(seeds = [b"treasury"], bump)]
    pub treasury: Account<'info, Treasury>,
}

pub fn get_treasury(ctx: Context<GetTreasury>) -> Result<()> {
    let treasury = &ctx.accounts.treasury;

    msg!("📜 Treasury Data:");
    msg!("Admins: {:?}", treasury.admins);
    msg!("Bump: {}", treasury.bump);
    msg!("Version: {}", treasury.version);

    Ok(())
}
