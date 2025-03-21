use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;
pub mod arguments;

use crate::arguments::metadata::Metadata;
// use crate::arguments::mint_album_args::MintAlbumArgs;
use crate::instructions::initialize_token_mint::InitializeTokenMintArgs;
use crate::arguments::purchase_release_args::PurchaseReleaseArgs;
use crate::arguments::initialize_user_args::InitializeUserArgs;

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

    pub fn initialize_token_mint(ctx: Context<InitializeTokenMint>, data: InitializeTokenMintArgs) -> Result<()> {
        instructions::initialize_token_mint::initialize_token_mint(ctx, data)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, mint_tokens_args: MintTokensArgs) -> Result<()> {
        instructions::mint_tokens::mint_tokens(ctx, mint_tokens_args)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, data: TransferTokensArgs) -> Result<()> {
        instructions::transfer_tokens::transfer_tokens(ctx, data)
    }

    pub fn add_metaplex_metadata(ctx: Context<AddMetaplexMetadata>, metadata: Metadata) -> Result<()> {
        instructions::add_metaplex_metadata::add_metaplex_metadata(ctx, metadata)
    }

    pub fn purchase_release(ctx: Context<PurchaseRelease>, data: PurchaseReleaseArgs) -> Result<()> {
        instructions::purchase_release::purchase_release(ctx, data)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, data: InitializeUserArgs) -> Result<()> {
        instructions::initialize_user::initialize_user(ctx, data)
    }

    pub fn close_user_account(ctx: Context<CloseUserAccount>, user_id: String) -> Result<()> {
        instructions::close_user_account::close_user_account(ctx, user_id)
    }

    pub fn update_admins(ctx: Context<UpdateAdmins>, action: AdminAction) -> Result<()> {
        instructions::update_admins::update_admins(ctx, action)
    }

    // pub fn mint_album(ctx: Context<MintAlbum>, data: MintAlbumArgs) -> Result<()> {
    //     instructions::mint_album::mint_album(ctx, data)
    // }
}

