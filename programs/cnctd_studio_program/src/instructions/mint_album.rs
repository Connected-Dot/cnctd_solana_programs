use anchor_lang::prelude::*;
use crate::state::album::Album;

pub fn mint_album(ctx: Context<MintAlbum>, album: Album) -> Result<()> {
    msg!("Minting Album...");
    msg!("Album ID: {}", album.album_id);
    msg!("Creator: {}", album.creator);
    msg!("Price: {} lamports", album.price);
    // msg!("Resale Allowed: {}", album.resale_allowed);

    msg!("Credits:");
    for credit in &album.credits {
        msg!("  - Artist Wallet: {}", credit.artist_wallet);
        msg!("    Percentage: {}%", credit.percentage);
    }

    Ok(())
}
