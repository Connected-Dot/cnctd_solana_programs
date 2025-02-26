use anchor_lang::prelude::*;

use crate::state::program_metadata::ProgramMetadata;

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        has_one = owner,  // Ensures only the owner can update
        seeds = [b"program_metadata"],
        bump
    )]
    pub metadata: Account<'info, ProgramMetadata>,

    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

pub fn update_metadata(ctx: Context<UpdateMetadata>, new_owner: Pubkey, new_version: u8) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    
    metadata.owner = new_owner;
    metadata.version = new_version;
    
    msg!("Program metadata updated. New Owner: {}, New Version: {}", new_owner, new_version);
    Ok(())
}
