use anchor_lang::prelude::*;

use crate::state::program_metadata::ProgramMetadata;

#[derive(Accounts)]
pub struct InitializeProgramMetadata<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 1 + 32, // Space for owner (32 bytes), version (1 byte), treasury_pda (32 bytes)
        seeds = [b"program_metadata"],
        bump
    )]
    pub metadata: Account<'info, ProgramMetadata>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_program_metadata(ctx: Context<InitializeProgramMetadata>, treasury_pda: Pubkey) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;

    metadata.owner = ctx.accounts.payer.key(); // The first signer is the owner
    metadata.version = 1; // Start at version 1
    metadata.treasury_pda = treasury_pda;

    msg!("Program metadata initialized with owner: {}", metadata.owner);
    
    Ok(())
}
