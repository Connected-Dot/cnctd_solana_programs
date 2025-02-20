pub use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(fee_bps: u16)]
pub struct InitTreasury<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 2 + 1, // Space for fee_bps (u16) + bump (u8)
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
