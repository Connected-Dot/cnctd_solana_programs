use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, Transfer, transfer, MintTo, mint_to},
    token_2022::Token2022,
    metadata::{
        create_metadata_accounts_v3, 
        CreateMetadataAccountsV3,
        Metadata
    }
};

use crate::{arguments::purchase_release_args::PurchaseReleaseArgs, errors::CnctdStudioError, state::treasury::Treasury};

#[derive(Accounts)]
pub struct PurchaseRelease<'info> {
    // Buyer
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: This is the buyer's USDC token account
    #[account(mut)]
    pub buyer_usdc_ata: AccountInfo<'info>,
    
    /// CHECK: This is the buyer's MUS1C token account (for rewards)
    #[account(mut)]
    pub buyer_music_ata: AccountInfo<'info>,
    
    /// CHECK: This is the buyer's NFT token account (for the album)
    #[account(mut)]
    pub buyer_nft_ata: AccountInfo<'info>,
    
    // Treasury
    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// CHECK: This is the treasury's USDC token account
    #[account(mut)]
    pub treasury_usdc_ata: AccountInfo<'info>,
    
    // Mints
    /// CHECK: This is the USDC mint
    pub usdc_mint: AccountInfo<'info>,
    
    /// CHECK: This is the MUS1C mint (reward token)
    pub music_mint: AccountInfo<'info>,
    
    /// CHECK: This is the CNCTD mint (creator reward token)
    pub cnctd_mint: AccountInfo<'info>,
    
    /// CHECK: This is the NFT mint for the album
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,
    
    /// CHECK: This is the NFT metadata account
    #[account(mut)]
    pub nft_metadata: AccountInfo<'info>,
    
    /// CHECK: This is the metaplex token metadata program
    pub token_metadata_program: AccountInfo<'info>,
    
    // System accounts
    pub token_program: Program<'info, Token>,
    pub token_2022_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    
    // Remaining accounts will include:
    // - Artist PDAs
    // - Artist USDC ATAs
    // - Artist CNCTD ATAs
}

pub fn purchase_release(
    ctx: Context<PurchaseRelease>,
    args: PurchaseReleaseArgs,
) -> Result<()> {
    let buyer = &ctx.accounts.buyer;
    let treasury = &ctx.accounts.treasury;

    msg!("Purchase album instruction called");
    msg!("Buyer: {:?}", buyer.key());
    msg!("Treasury: {:?}", treasury.key());
    msg!("Release Args: {:?}", args);
    
    // 1. Transfer USDC fee to treasury
    // transfer(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         Transfer {
    //             from: ctx.accounts.buyer_usdc_ata.clone(),
    //             to: ctx.accounts.treasury_usdc_ata.clone(),
    //             authority: buyer.to_account_info(),
    //         },
    //     ),
    //     args.treasury_fee,
    // )?;
    
    // 2. Transfer USDC to each artist based on pre-calculated splits
    // let remaining_accounts = &ctx.remaining_accounts;
    
    // for (i, split) in args.payment_splits.iter().enumerate() {
    //     let recipient_ata = &remaining_accounts[i];
        
    //     transfer(
    //         CpiContext::new(
    //             ctx.accounts.token_program.to_account_info(),
    //             Transfer {
    //                 from: ctx.accounts.buyer_usdc_ata.clone(),
    //                 to: recipient_ata.clone(),
    //                 authority: buyer.to_account_info(),
    //             },
    //         ),
    //         split.amount,
    //     )?;
    // }
    
    // 3. Mint NFT to buyer
    // Implementation for NFT minting
    // (This would be added once you're ready to implement this part)
    
    // msg!("Album purchase successful!");
    Ok(())
}
