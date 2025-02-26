use anchor_lang::prelude::*;

use crate::state::artist_pda::ArtistPDA;

#[derive(Accounts)]
pub struct InitializeArtist<'info> {
    /// CHECK: InitializeArtist
    #[account(
        init, 
        payer = signer, 
        space = 8 + std::mem::size_of::<ArtistPDA>(), 
        seeds = [b"user", signer.key().as_ref()], 
        bump,
    )]
    pub artist_pda: Account<'info, ArtistPDA>,

    /// The wallet that initializes and funds the transaction
    #[account(mut, signer)]
    pub signer: Signer<'info>,  

    /// CHECK: This is the platform admin account
    #[account(
        address = PLATFORM_ADMIN_PUBKEY 
    )]
    pub platform_admin: UncheckedAccount<'info>, 

    pub system_program: Program<'info, System>,
}

pub fn initialize_artist(ctx: Context<InitializeArtist>) -> Result<()> {
    let artist_pda = &mut ctx.accounts.artist_pda;
    let bump = ctx.bumps.artist_pda;

    // Hardcoded Values
    artist_pda.admin = ctx.accounts.platform_admin.key();
    artist_pda.auth = Some(ctx.accounts.signer.key());
    artist_pda.usdc_ata = Pubkey::new_from_array([1; 32]);  
    artist_pda.usdc_cust = Some(Pubkey::new_from_array([2; 32]));  
    artist_pda.cnctd_ata = Pubkey::new_from_array([3; 32]);  
    artist_pda.cnctd_cust = Some(Pubkey::new_from_array([4; 32]));  
    artist_pda.fees_waived = 0;
    artist_pda.waived_count = 0;
    artist_pda.bump = bump;

    msg!("Hardcoded ArtistPDA initialized: {:?}", ctx.accounts.artist_pda.key());

    Ok(())
}
