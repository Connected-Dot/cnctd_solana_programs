use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub fee_bps: u16,  // Fee percentage in basis points (e.g., 500 = 5%)
    pub bump: u8,      // PDA bump
}

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
