use anchor_lang::{prelude::*, solana_program};
use anchor_lang::solana_program::sysvar;
use anchor_spl::metadata::mpl_token_metadata;
use anchor_spl::metadata::mpl_token_metadata::instructions::{CreateV1, UpdateV1, UpdateV1InstructionArgs};
use anchor_spl::metadata::mpl_token_metadata::types::{CollectionToggle, Creator, Data, TokenStandard};
use anchor_spl::token_2022::Token2022;

use crate::arguments::metadata::Metadata;
use crate::errors::CnctdStudioError;
use crate::state::treasury::Treasury;

#[derive(Accounts)]
pub struct AddMetaplexMetadata<'info> {
    /// CHECK: Token-2022 mint account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Metaplex metadata account (PDA derived from mint)
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>, // PDA that owns mint & metadata authority

    /// CHECK: Treasury admin account
    #[account(signer)]
    pub payer: AccountInfo<'info>,

    /// CHECK: Metaplex Token Metadata Program
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,

    /// CHECK: Instructions sysvar (Manually handled)
    #[account(address = solana_program::sysvar::instructions::ID)]
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn add_metaplex_metadata(
    ctx: Context<AddMetaplexMetadata>,
    metadata: Metadata,
) -> Result<()> {
    if !ctx.accounts.treasury.is_admin(&ctx.accounts.payer.key()) {
        return Err(CnctdStudioError::Unauthorized.into());
    }

    let bump = ctx.accounts.treasury.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[b"treasury",&[bump]]];

    let metadata_pda = ctx.accounts.metadata.key();
    let mint = ctx.accounts.mint.key();
    let treasury_pda = ctx.accounts.treasury.key();
    let payer = ctx.accounts.payer.key();

    msg!("Adding Metaplex metadata to mint: {:?}", mint);
    msg!("Metadata PDA: {:?}", metadata_pda);
    msg!("Treasury PDA: {:?}", treasury_pda);
    msg!("Payer: {:?}", payer);

    let args = mpl_token_metadata::instructions::CreateV1InstructionArgs {
        name: metadata.name.clone(),
        symbol: metadata.symbol.clone(),
        uri: metadata.uri.clone(),
        seller_fee_basis_points: metadata.seller_fee_basis_points,
        creators: metadata.creators.clone().map(|c| c.iter().map(|creator| Creator {
            address: creator.address,
            verified: creator.verified,
            share: creator.share,
        }).collect()),
        primary_sale_happened: false,
        is_mutable: true,
        token_standard: TokenStandard::FungibleAsset,
        collection: None,
        uses: None,
        collection_details: None,
        rule_set: None,
        decimals: Some(6),
        print_supply: None,
    };

    // Check if metadata exists
    let metadata_account_info = ctx.accounts.metadata.to_account_info();
    if metadata_account_info.data_is_empty() {
        msg!("Metadata does not exist, creating new metadata...");
        let cpi_instruction = CreateV1 {
            metadata: metadata_pda,
            master_edition: None,
            mint: (mint, false),
            authority: treasury_pda,
            payer,
            update_authority: (treasury_pda, true),
            system_program: ctx.accounts.system_program.key(),
            sysvar_instructions: sysvar::instructions::ID,
            spl_token_program: Some(ctx.accounts.token_program.key()),
        }
        .instruction(args);

        anchor_lang::solana_program::program::invoke_signed(
            &cpi_instruction,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.sysvar_instructions.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            signer_seeds,
        )?;
    } else {
        msg!("Metadata exists, updating metadata...");
        
        let cpi_instruction = UpdateV1 {
            metadata: metadata_pda,
            authority: treasury_pda, // Update authority must be set here
            delegate_record: None,
            token: None, // Not updating a token account
            mint,
            edition: None, // No edition for fungible assets
            payer,
            system_program: ctx.accounts.system_program.key(),
            sysvar_instructions: sysvar::instructions::ID,
            authorization_rules_program: None,
            authorization_rules: None,
        }.instruction(UpdateV1InstructionArgs {
            new_update_authority: None, // Keep the same update authority
            data: Some(Data {
                name: metadata.name,
                symbol: metadata.symbol,
                uri: metadata.uri,
                seller_fee_basis_points: metadata.seller_fee_basis_points,
                creators: metadata.creators.map(|c| c.iter().map(|creator| Creator {
                    address: creator.address,
                    verified: creator.verified,
                    share: creator.share,
                }).collect()),
            }),
            primary_sale_happened: None,
            is_mutable: Some(true),
            ..Default::default()
        });

        anchor_lang::solana_program::program::invoke_signed(
            &cpi_instruction,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.mint.to_account_info(), // ✅ MINT IS REQUIRED
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.sysvar_instructions.to_account_info(),
            ],
            signer_seeds,
        )?;
        
    }

    msg!("Metaplex metadata added to mint: {:?}", ctx.accounts.mint.key());

    Ok(())
}
