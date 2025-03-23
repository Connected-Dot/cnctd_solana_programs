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

    #[account(
        mut, // Must be mutable since it receives and sends SOL
        constraint = treasury.is_admin(&admin.key())
    )]
    pub admin: Signer<'info>,
    
    /// CHECK: This is the treasury's USDC token account
    #[account(mut)]
    pub treasury_usdc_ata: AccountInfo<'info>,

    /// CHECK: This is the treasury's CNCTD token account
    #[account(mut)]
    pub treasury_cnctd_ata: AccountInfo<'info>,

    /// CHECK: This is the treasury's MUS1C token account
    #[account(mut)]
    pub treasury_music_ata: AccountInfo<'info>,
    
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
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_usdc_ata.clone(),
                to: ctx.accounts.treasury_usdc_ata.clone(),
                authority: buyer.to_account_info(),
            },
        ),
        args.treasury_fee,
    )?;
    
    // 2. Transfer USDC to each artist based on pre-calculated splits
    let remaining_accounts = &ctx.remaining_accounts;

    let bump = ctx.accounts.treasury.bump;
    let seeds: &[&[u8]] = &[b"treasury", &[bump]];
    let treasury_signer_seeds: &[&[&[u8]]] = &[seeds];
    let cnctd_reward = args.cnctd_reward;
    
    for (i, split) in args.payment_splits.iter().enumerate() {
        let recipient_ata = &remaining_accounts[i];
        let recipient_cnctd_ata = todo!();
        
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer_usdc_ata.clone(),
                    to: recipient_ata.clone(),
                    authority: buyer.to_account_info(),
                },
            ),
            split.amount,
        )?;

        if cnctd.reward.is_some() {
            transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_2022_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.treasury_cnctd_ata.clone(),
                        to: recipient_cnctd_ata.clone(),
                        authority: treasury.to_account_info(),
                    },
                    treasury_signer_seeds
                ),
                cnctd_reward.unwrap()
            )
        }
        
    }
    
    // 3. Mint NFT to buyer
    // Implementation for NFT minting
    // (This would be added once you're ready to implement this part)
    
    // msg!("Album purchase successful!");

    let buyer_music_ata = ctx.accounts.buyer_music_ata;
    let music_reward = args.music_reward;

    if music_reward.is_some() {
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_2022_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury_music_ata.clone(),
                    to: buyer_music_ata.clone(),
                    authority: treasury.to_account_info(),
                },
                treasury_signer_seeds
            ),
            music_reward.unwrap()
        )
    }

    let treasury_info = ctx.accounts.treasury.to_account_info();
    let admin_info = ctx.accounts.admin.to_account_info();
    let fee_compensation = args.fee_compensation.unwrap_or(0);

    let total_reimbursement = rent_lamports + fee_compensation;
    
    **treasury_info.try_borrow_mut_lamports()? -= total_reimbursement;
    **admin_info.try_borrow_mut_lamports()? += total_reimbursement;
    
    msg!("Reimbursed admin {} lamports from treasury", total_reimbursement);
    msg!("Final admin balance: {}", ctx.accounts.admin.lamports());
    msg!("Final treasury balance: {}", ctx.accounts.treasury.to_account_info().lamports());

    Ok(())
}
