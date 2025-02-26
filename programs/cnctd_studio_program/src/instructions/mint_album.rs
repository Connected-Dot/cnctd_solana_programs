use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::mpl_token_metadata,
    token_2022::Token2022,
};

use crate::{arguments::mint_album_args::MintAlbumArgs, state::{artist_pda::ArtistPDA, treasury::Treasury}};

#[derive(Accounts)]
pub struct MintAlbum<'info> {
    /// CHECK: Album NFT mint account (validated in the function)
    #[account(
        mut,
        seeds = [b"album"],
        bump = album_mint.bump
    )]
    pub album_mint: AccountInfo<'info>, 

    #[account(
        init,
        seeds = [],
        bump,
        payer = owner,
        space = 8 + size_of::<SomeState>()
        )]
    pub pda_account: AccountLoader<'info, SomeState>,

    /// CHECK: Metaplex Metadata Account (validated in the function)
    #[account(mut)]
    pub metadata_pda: UncheckedAccount<'info>,

    /// CHECK: Master Edition Account (validated in the function)
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// Treasury PDA (Holds album NFTs, receives sales)
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump 
    )]
    pub treasury: Account<'info, Treasury>,

    /// Artist PDA (Ensures the artist is registered)
    #[account(
        mut,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub artist_pda: Account<'info, ArtistPDA>,

    /// CHECK: Associated Token Account for Treasury (validated in the function)
    #[account(mut)]
    pub treasury_ata: AccountInfo<'info>,  

    /// The artist or admin submitting the transaction
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Metaplex Token Metadata Program
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

    /// Token-2022 Program
    pub token_program: Program<'info, Token2022>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_album(ctx: Context<MintAlbum>, data: MintAlbumArgs) -> Result<()> {
    let treasury = &ctx.accounts.treasury;
    let artist_pda = &ctx.accounts.artist_pda;
    let signer = &ctx.accounts.signer;
    let treasury_ata = &ctx.accounts.treasury_ata;
    let album_mint = &ctx.accounts.album_mint;
    let metadata_pda = &ctx.accounts.metadata_pda;
    let master_edition = &ctx.accounts.master_edition;

    msg!("Got MintAlbumArgs: {:?}", data);
    msg!("Treasury: {:?}", treasury.to_account_info());
    msg!("Artist PDA: {:?}", artist_pda.to_account_info());
    msg!("Signer: {:?}", signer);
    msg!("Treasury ATA: {:?}", treasury_ata);
    msg!("Album Mint: {:?}", album_mint);
    msg!("Metadata PDA: {:?}", metadata_pda);
    msg!("Master Edition: {:?}", master_edition);


    Ok(())
}