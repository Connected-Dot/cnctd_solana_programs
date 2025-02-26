use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;
pub mod arguments;

use crate::arguments::metadata::Metadata;
use crate::arguments::mint_album_args::MintAlbumArgs;

use instructions::*;

declare_id!("CSPd6eauKNBXfrQnKmqrHKEjt6xtW7mgzmfV2XPfiy5i");

#[program]
pub mod cnctd_solana_program {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     instructions::initialize::initialize(ctx)
    // }

    // pub fn close_treasury(ctx: Context<CloseTreasury>) -> Result<()> {
    //     instructions::close_treasury::close_treasury(ctx)
    // }

    pub fn initialize_token_mint(ctx: Context<InitializeTokenMint>) -> Result<()> {
        instructions::initialize_token_mint::initialize_token_mint(ctx)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, mint_amount: u64) -> Result<()> {
        instructions::mint_tokens::mint_tokens(ctx, mint_amount)
    }

    pub fn add_metaplex_metadata(ctx: Context<AddMetaplexMetadata>, metadata: Metadata) -> Result<()> {
        instructions::add_metaplex_metadata::add_metaplex_metadata(ctx, metadata)
    }

    pub fn initialize_artist(ctx: Context<InitializeArtist>) -> Result<()> {
        instructions::initialize_artist::initialize_artist(ctx)
    }

    pub fn mint_album(ctx: Context<MintAlbum>, data: MintAlbumArgs) -> Result<()> {
        instructions::mint_album::mint_album(ctx, data)
    }
}

