use anchor_lang::prelude::*;
use anchor_spl::{token::{spl_token, Mint, TokenAccount}, token_2022::MintTo as MintTo2022};

use crate::{
    arguments::release_access::CompleteReleaseAccessArgs,
    errors::CnctdStudioError,
    state::{release_escrow::ReleaseEscrow, treasury::Treasury, user_pda::UserPDA},
    utils::UuidFormatting,
};

#[derive(Accounts)]
#[instruction(args: CompleteReleaseAccessArgs)]
pub struct CompleteReleaseAccess<'info> {
    #[account(
        mut,
        constraint = treasury.is_admin(&admin.key()) @ CnctdStudioError::Unauthorized
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", args.buyer_id.as_ref()],
        bump
    )]
    pub buyer: Account<'info, UserPDA>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    pub usdc_mint: Account<'info, Mint>,
    
    /// CHECK: CNCTD mint, using token 2022 program
    #[account(mut)]
    pub cnctd_mint: UncheckedAccount<'info>,
    
    /// CHECK: MUSIC mint, using token 2022 program
    #[account(mut)]
    pub music_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        close = treasury,
        seeds = [
            b"release_escrow",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump,
        constraint = escrow.fulfilled == true @ CnctdStudioError::EscrowNotFulfilled,
        constraint = escrow.payments_fulfilled == true @ CnctdStudioError::PaymentsNotFulfilled,
        constraint = escrow.nft_minted == true @ CnctdStudioError::NFTNotMinted,
    )]
    pub escrow: Account<'info, ReleaseEscrow>,

    /// CHECK: Escrow USDC token account
    #[account(mut)]
    pub escrow_usdc_ata: Account<'info, TokenAccount>,
    
    /// CHECK: Buyer MUSIC token account
    #[account(mut)]
    pub buyer_music_ata: UncheckedAccount<'info>,

    /// CHECK: Standard token program (should be TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)
    pub standard_token_program: UncheckedAccount<'info>,
    
    /// CHECK: Custom token program (for CNCTD and MUSIC)
    pub token_2022_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn complete<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CompleteReleaseAccess<'info>>,
    args: CompleteReleaseAccessArgs
) -> Result<()> {
    msg!("Complete Release Instruction");
    
    // 1. Mint MUSIC tokens to buyer based on the amount they paid
    mint_music_to_buyer(&ctx.accounts, &args)?;
    
    // 2. Mint CNCTD rewards to artists based on payment splits
    mint_cnctd_to_artists(&ctx.accounts, ctx.remaining_accounts, &args)?;
    
    // 3. Close the escrow USDC ATA and return lamports to treasury
    close_escrow_token_account(&ctx.accounts)?;
    
    // 4. Reimburse admin for transaction fees if specified
    ctx.accounts.treasury.reimburse_admin(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.admin.to_account_info(),
        args.fee_compensation,
        None, // No rent to reimburse as all accounts should already exist
    )?;

    msg!("Release completed successfully");
    Ok(())
}

// Helper function to mint MUSIC tokens to buyer
fn mint_music_to_buyer(accounts: &CompleteReleaseAccess, args: &CompleteReleaseAccessArgs) -> Result<()> {
    let music_amount = accounts.escrow.total_amount; // Same as USDC paid
    msg!("Minting {} MUSIC tokens to buyer", music_amount);
    
    // Get treasury signer seeds
    let treasury_signer_seeds: &[&[u8]] = &[b"treasury", &[accounts.treasury.bump]];
    
    // Using the MintTo2022 structure for token-2022 program
    let cpi_accounts = MintTo2022 {
        mint: accounts.music_mint.to_account_info(),
        to: accounts.buyer_music_ata.to_account_info(),
        authority: accounts.treasury.to_account_info(),
    };
    
    // Use token_2022::mint_to for minting token-2022 tokens
    anchor_spl::token_2022::mint_to(
        CpiContext::new_with_signer(
            accounts.token_2022_program.to_account_info(),
            cpi_accounts,
            &[treasury_signer_seeds],
        ),
        music_amount,
    )?;
    
    Ok(())
}

// Helper function to mint CNCTD tokens to artists
fn mint_cnctd_to_artists<'a, 'b, 'c, 'info>(
    accounts: &CompleteReleaseAccess<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],
    args: &CompleteReleaseAccessArgs
) -> Result<()> {
    // Calculate number of artists (payment splits excluding treasury fee)
    let artist_count = accounts.escrow.payment_splits.len();
    msg!("Minting CNCTD rewards to {} artists", artist_count);
    
    if artist_count == 0 {
        msg!("No artists to mint CNCTD rewards to");
        return Ok(());
    }
    
    // Calculate CNCTD reward per artist
    let total_cnctd_reward = accounts.escrow.total_amount; // Total USDC amount paid for release
    let cnctd_per_artist = total_cnctd_reward / (artist_count as u64);
    msg!("Each artist will receive {} CNCTD tokens", cnctd_per_artist);
    
    // Get treasury signer seeds
    let treasury_signer_seeds: &[&[u8]] = &[b"treasury", &[accounts.treasury.bump]];
    
    // Iterator for remaining accounts (artist CNCTD ATAs)
    let mut remaining_accounts_iter = remaining_accounts.iter();
    
    // Process each payment split
    for (i, split) in accounts.escrow.payment_splits.iter().enumerate() {
        // Get the artist's CNCTD token account from remaining accounts
        let artist_cnctd_ata = next_account_info(&mut remaining_accounts_iter)?;
        
        msg!("Minting {} CNCTD to artist {}", cnctd_per_artist, i + 1);
        
        // Using the MintTo2022 structure for token-2022 program
        let cpi_accounts = MintTo2022 {
            mint: accounts.cnctd_mint.to_account_info(),
            to: artist_cnctd_ata.clone(),
            authority: accounts.treasury.to_account_info(),
        };
        
        // Use token_2022::mint_to for minting token-2022 tokens
        anchor_spl::token_2022::mint_to(
            CpiContext::new_with_signer(
                accounts.token_2022_program.to_account_info(),
                cpi_accounts,
                &[treasury_signer_seeds],
            ),
            cnctd_per_artist,
        )?;
    }
    
    Ok(())
}

// Helper function to close escrow USDC token account
fn close_escrow_token_account(accounts: &CompleteReleaseAccess) -> Result<()> {
    msg!("Closing escrow USDC token account");
    
    let release_seed = accounts.escrow.release_id.to_7_byte_seed();
    let buyer_seed = accounts.escrow.buyer_id.to_7_byte_seed();
    
    // Get escrow signer seeds
    let escrow_seeds = &[
        b"release_escrow",
        release_seed.as_ref(),
        buyer_seed.as_ref(),
        &[accounts.escrow.bump]
    ];
    
    // Create custom CPI instruction to close the token account
    let ix = spl_token::instruction::close_account(
        &accounts.standard_token_program.key(),  // Standard token program
        &accounts.escrow_usdc_ata.key(),         // Account to close
        &accounts.treasury.key(),                // Destination for lamports
        &accounts.escrow.key(),                  // Authority (escrow PDA)
        &[]                                      // Signers
    )?;
    
    // Invoke the instruction with escrow PDA as signer
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            accounts.standard_token_program.to_account_info(),
            accounts.escrow_usdc_ata.to_account_info(),
            accounts.treasury.to_account_info(),
            accounts.escrow.to_account_info(),
        ],
        &[escrow_seeds]
    )?;
    
    Ok(())
}