use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}, token_2022::Token2022};

use crate::{arguments::release::PurchaseAccessArgs, errors::CnctdStudioError, state::{release_access::ReleaseAccess, treasury::Treasury, user_pda::UserPDA}, utils::UuidFormatting};

#[derive(Accounts)]
#[instruction(args: PurchaseAccessArgs)]
pub struct PurchaseAccess<'info> {
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

    #[account(mut)]
    pub buyer_usdc_ata: Account<'info, TokenAccount>,

    /// CHECK: Buyer MUSIC token account
    #[account(mut)]
    pub buyer_music_ata: UncheckedAccount<'info>,

    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub treasury_usdc_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        space = ReleaseAccess::space(),
        seeds = [
            b"release_access",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump,
    )]
    pub release_access: Account<'info, ReleaseAccess>, 

    pub usdc_mint: Account<'info, Mint>,
    
    /// CHECK: CNCTD mint, using token 2022 program
    #[account(mut)]
    pub cnctd_mint: UncheckedAccount<'info>,
    
    /// CHECK: MUSIC mint, using token 2022 program
    #[account(mut)]
    pub music_mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn purchase_access<'a, 'b, 'c, 'info>(
    mut ctx: Context<'a, 'b, 'c, 'info, PurchaseAccess<'info>>,
    args: PurchaseAccessArgs,
) -> Result<()> {
    msg!("Purchase Access Instruction");

    // 1. Initialize the release access PDA
    initialize_release_access(&mut ctx.accounts, &args)?;

    // 2. Pay the treasury fee
    pay_treasury_fee(&mut ctx.accounts, &args)?;

    // 3. Pay each artist based on payment splits
    pay_artists(&mut ctx.accounts, ctx.remaining_accounts, &args)?;

    // 4. Mint MUSIC tokens to buyer
    mint_music_to_buyer(&mut ctx.accounts, &args)?;

    // 5. Mint CNCTD tokens to artists
    mint_cnctd_to_artists(&mut ctx.accounts, ctx.remaining_accounts, &args)?;

    // 6. Reimburse admin for transaction fees if specified
    reimburse_admin(&mut ctx.accounts, &args)?;

    msg!("Access purchase completed successfully");

    Ok(())
}

fn initialize_release_access(accounts: &mut PurchaseAccess, args: &PurchaseAccessArgs) -> Result<()> {
    msg!("Initializing Release Access PDA");
    
    let release_access = &mut accounts.release_access;
    
    // Populate the release_access PDA with essential information
    release_access.release_id = args.release_id.clone();
    msg!("Release ID: {}", release_access.release_id);
    
    release_access.buyer_id = args.buyer_id.clone();
    msg!("Buyer ID: {}", release_access.buyer_id);
    
    release_access.created_at = args.created_at;
    msg!("Created At: {}", release_access.created_at);
    
    release_access.expiration_date = args.expiration_date;
    msg!("Expiration Date: {:?}", release_access.expiration_date);
    
    Ok(())
}

fn pay_treasury_fee(accounts: &mut PurchaseAccess, args: &PurchaseAccessArgs) -> Result<()> {
    msg!("Paying treasury fee: {} USDC", args.treasury_fee);
    
    // Skip if treasury fee is zero
    if args.treasury_fee == 0 {
        msg!("Treasury fee is zero, skipping payment");
        return Ok(());
    }
    
    let treasury_usdc_ata = &accounts.treasury_usdc_ata;

    
    // Transfer from buyer to treasury
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: accounts.buyer_usdc_ata.to_account_info(),
                to: treasury_usdc_ata.to_account_info(),
                authority: accounts.buyer.to_account_info(),
            },
            &[&[b"user", args.buyer_id.as_ref(), &[accounts.buyer.bump]]]
        ),
        args.treasury_fee
    )?;
    
    msg!("Treasury fee payment successful");

    Ok(())
}

fn pay_artists<'a, 'b, 'c, 'info>(
    accounts: &mut PurchaseAccess<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],
    args: &PurchaseAccessArgs
) -> Result<()> {
    msg!("Processing payments to {} artists", args.payment_splits.len());
    
    // Skip if no payment splits
    if args.payment_splits.is_empty() {
        msg!("No artist payments to process");
        return Ok(());
    }
    
    // Process each payment split
    for (i, split) in args.payment_splits.iter().enumerate() {
        // Get the recipient's token account from remaining accounts
        let recipient_ata = &remaining_accounts[i];
        
        // Verify the recipient account matches what's stored in the payment split
        require!(
            recipient_ata.key() == split.recipient_usdc_ata,
            CnctdStudioError::InvalidPaymentReceiver
        );
        
        msg!("Paying {} USDC to recipient {}", split.amount, split.recipient_usdc_ata);
        
        // Skip if payment amount is zero
        if split.amount == 0 {
            msg!("Payment amount is zero, skipping");
            continue;
        }
        
        // Transfer payment to artist
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: accounts.buyer_usdc_ata.to_account_info(),
                    to: recipient_ata.clone(),
                    authority: accounts.buyer.to_account_info(),
                },
                &[&[b"user", args.buyer_id.as_ref(), &[accounts.buyer.bump]]]
            ),
            split.amount
        )?;
    }
    
    msg!("Artist payments completed successfully");

    Ok(())
}

fn mint_music_to_buyer(accounts: &mut PurchaseAccess, args: &PurchaseAccessArgs) -> Result<()> {
    // Calculate total payment amount (treasury fee + all artist payments)
    let total_payment = args.treasury_fee + args.payment_splits.iter().map(|split| split.amount).sum::<u64>();
    msg!("Minting {} MUSIC tokens to buyer", total_payment);
    
    // Skip if total payment is zero
    if total_payment == 0 {
        msg!("Total payment is zero, skipping MUSIC minting");
        return Ok(());
    }
    
    // Get treasury signer seeds
    let treasury_seeds: &[&[u8]] = &[b"treasury", &[accounts.treasury.bump]];
    
    // Mint MUSIC tokens to buyer
    anchor_spl::token_2022::mint_to(
        CpiContext::new_with_signer(
            accounts.token_2022_program.to_account_info(),
            anchor_spl::token_2022::MintTo {
                mint: accounts.music_mint.to_account_info(),
                to: accounts.buyer_music_ata.to_account_info(),
                authority: accounts.treasury.to_account_info(),
            },
            &[treasury_seeds]
        ),
        total_payment
    )?;
    
    msg!("MUSIC tokens minted successfully");

    Ok(())
}

fn mint_cnctd_to_artists<'a, 'b, 'c, 'info>(
    accounts: &mut PurchaseAccess<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],
    args: &PurchaseAccessArgs
) -> Result<()> {
    // Calculate number of artists (payment splits)
    let artist_count = args.payment_splits.len();
    
    if artist_count == 0 {
        msg!("No artists to mint CNCTD rewards to");
        return Ok(());
    }
    
    // Calculate total payment amount (including treasury fee)
    let total_payment = args.treasury_fee + args.payment_splits.iter().map(|split| split.amount).sum::<u64>();
    
    // Calculate proportional rewards for each artist
    // Each artist gets rewarded based on their proportion of the artist payments, but applied to the total payment
    let total_artist_payments = args.payment_splits.iter().map(|split| split.amount).sum::<u64>();
    
    msg!("Minting CNCTD rewards to {} artists", artist_count);
    
    // Get treasury signer seeds
    let treasury_seeds: &[&[u8]] = &[b"treasury", &[accounts.treasury.bump]];
    
    // Skip the artist USDC ATAs to get to CNCTD ATAs
    let cnctd_atas_start = artist_count;
    
    // Process each payment split
    for (i, split) in args.payment_splits.iter().enumerate() {
        // Get the artist's CNCTD token account from the remaining accounts
        let artist_cnctd_ata = &remaining_accounts[cnctd_atas_start + i];
        
        // Verify the recipient CNCTD account matches what's in the payment split
        require!(
            artist_cnctd_ata.key() == split.recipient_cnctd_ata,
            CnctdStudioError::InvalidPaymentReceiver
        );
        
        // Calculate CNCTD reward based on artist's proportion of the total payment
        // This gives them their share of the entire payment including treasury fee
        let artist_proportion = if total_artist_payments > 0 {
            (split.amount as f64) / (total_artist_payments as f64)
        } else {
            0.0
        };
        
        let cnctd_reward = if total_artist_payments > 0 {
            (artist_proportion * (total_payment as f64)).round() as u64
        } else {
            0
        };
        
        // Skip if reward amount is zero
        if cnctd_reward == 0 {
            msg!("CNCTD reward amount is zero for artist {}, skipping", i);
            continue;
        }
        
        msg!("Minting {} CNCTD to artist {} ({}% of total)", 
            cnctd_reward, i + 1, (artist_proportion * 100.0).round() as u64);
        
        // Mint CNCTD tokens to artist
        anchor_spl::token_2022::mint_to(
            CpiContext::new_with_signer(
                accounts.token_2022_program.to_account_info(),
                anchor_spl::token_2022::MintTo {
                    mint: accounts.cnctd_mint.to_account_info(),
                    to: artist_cnctd_ata.clone(),
                    authority: accounts.treasury.to_account_info(),
                },
                &[treasury_seeds]
            ),
            cnctd_reward
        )?;
    }
    
    msg!("CNCTD rewards minted successfully");
    Ok(())
}

fn reimburse_admin(accounts: &mut PurchaseAccess, args: &PurchaseAccessArgs) -> Result<()> {
    // Calculate rent for ReleaseAccess account
    let rent = Rent::get()?;
    let release_access_rent = rent.minimum_balance(ReleaseAccess::space());
    
    msg!("Reimbursing admin for transaction costs");
    
    // Use the treasury's reimburse_admin method
    accounts.treasury.reimburse_admin(
        &accounts.treasury.to_account_info(),
        &accounts.admin.to_account_info(),
        args.fee_compensation,
        Some(release_access_rent),
    )?;
    
    msg!("Admin reimbursed successfully");
    Ok(())
}