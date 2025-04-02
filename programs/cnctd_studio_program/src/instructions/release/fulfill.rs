use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata}, token::{Mint, Token, TokenAccount}};

use crate::{arguments::{metadata::Creator, release::FulfillReleaseArgs,}, errors::CnctdStudioError, state::{release_escrow::ReleaseEscrow, treasury::Treasury, user_pda::UserPDA}, utils::UuidFormatting};

#[derive(Accounts)]
#[instruction(args: FulfillReleaseArgs)]
pub struct FulfillRelease<'info> {
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
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub treasury_usdc_ata: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            b"release_escrow",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump
    )]
    pub escrow: Account<'info, ReleaseEscrow>,

    #[account(mut)]
    pub escrow_usdc_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 0,
        mint::authority = treasury,
        mint::freeze_authority = treasury, 
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_nft_ata: Account<'info, TokenAccount>,

    /// CHECK: This is the metadata account that will be created
    #[account(
        mut,
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(), 
            nft_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: This is the Metaplex program
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn fulfill<'a, 'b, 'c, 'info>(
    mut ctx: Context<'a, 'b, 'c, 'info, FulfillRelease<'info>>,
    args: FulfillReleaseArgs
) -> Result<()> {
    msg!("Fulfill Release Instruction");
    
    // 1. Pay the treasury fee from escrow
    pay_treasury_fee(&mut ctx.accounts, &args)?;

    // 2. Pay each artist based on payment splits
    pay_artists(&mut ctx.accounts, ctx.remaining_accounts, &args)?;

    let rent_sysvar = Rent::get()?;
    let total_rent = calculate_fulfill_rent_cost(&rent_sysvar);
    msg!("Calculated total rent: {} lamports", total_rent);
    // 3. Mint NFT to buyer and create metadata
    mint_nft(&mut ctx.accounts, &args)?;

    // 4. Mark escrow as fulfilled
    let escrow = &mut ctx.accounts.escrow;
    escrow.payments_fulfilled = true;
    // escrow.nft_minted is already set in mint_nft function
    escrow.fulfilled = true;
    
    // 5. Reimburse admin for transaction fees if specified
    ctx.accounts.treasury.reimburse_admin(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.admin.to_account_info(),
        args.fee_compensation,
        Some(total_rent), // reimburse for the rent calculated in mint_nft
    )?;


    msg!("Release fulfilled successfully");
    Ok(())
}

// Helper function to pay treasury fee
fn pay_treasury_fee(accounts: &mut FulfillRelease, args: &FulfillReleaseArgs) -> Result<()> {
    msg!("Paying treasury fee: {} USDC", accounts.escrow.treasury_fee);
    
    let release_seed = args.release_id.to_7_byte_seed();
    let buyer_seed = args.buyer_id.to_7_byte_seed();
    // Transfer from escrow to treasury
    let escrow_seeds = &[
        b"release_escrow",
        release_seed.as_ref(),
        buyer_seed.as_ref(),
        &[accounts.escrow.bump]
    ];
    
    // CPI to token program to transfer funds
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: accounts.escrow_usdc_ata.to_account_info(),
                to: accounts.treasury_usdc_ata.to_account_info(),
                authority: accounts.escrow.to_account_info(),
            },
            &[escrow_seeds]
        ),
        accounts.escrow.treasury_fee
    )?;
    
    Ok(())
}

// Helper function to pay artists - with explicit lifetimes
fn pay_artists<'a, 'b, 'c, 'info>(
    accounts: &mut FulfillRelease<'info>,
    remaining_accounts: &'a [AccountInfo<'info>],  // Note the explicit lifetimes
    args: &FulfillReleaseArgs
) -> Result<()> {
    msg!("Processing payments to {} artists", accounts.escrow.payment_splits.len());
    
    let release_seed = args.release_id.to_7_byte_seed();
    let buyer_seed = args.buyer_id.to_7_byte_seed();
    // Get escrow signer seeds
    let escrow_seeds = &[
        b"release_escrow",
        release_seed.as_ref(),
        buyer_seed.as_ref(),
        &[accounts.escrow.bump]
    ];
    
    // Iterator for remaining accounts (artist payment ATAs)
    let mut remaining_accounts_iter = remaining_accounts.iter();
    
    // Process each payment split
    for split in &accounts.escrow.payment_splits {
        // Get the recipient's token account from remaining accounts
        let recipient_ata = next_account_info(&mut remaining_accounts_iter)?;
        
        // Verify the recipient account matches what's stored in the escrow
        require!(
            recipient_ata.key() == split.recipient_usdc_ata,
            CnctdStudioError::InvalidPaymentReceiver
        );
        
        msg!("Paying {} USDC to recipient {}", split.amount, split.recipient_usdc_ata);
        
        // Transfer payment to artist
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: accounts.escrow_usdc_ata.to_account_info(),
                    to: recipient_ata.clone(),
                    authority: accounts.escrow.to_account_info(),
                },
                &[escrow_seeds]
            ),
            split.amount
        )?;
    }
    
    Ok(())
}

// Helper function to mint NFT and create metadata
fn mint_nft(accounts: &mut FulfillRelease, args: &FulfillReleaseArgs) -> Result<()> {
    msg!("Minting NFT to buyer");
    
    // 1. Mint one token to the buyer
    let treasury_seeds: &[&[u8]] = &[b"treasury", &[accounts.treasury.bump]];
    
    // Mint 1 token (NFT is non-fungible, so amount = 1)
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: accounts.nft_mint.to_account_info(),
                to: accounts.buyer_nft_ata.to_account_info(),
                authority: accounts.treasury.to_account_info(),
            },
            &[treasury_seeds]
        ),
        1
    )?;
    
    // 2. Create metadata for the NFT
    msg!("Creating NFT metadata");
    
    // Convert creators to Metaplex format
    let creators = Creator::multiple_to_metaplex_creators(
        args.creators.clone(),
        &accounts.treasury.key()
    );
    
     // Create DataV2 struct for metadata
     let data = mpl_token_metadata::types::DataV2 {
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        uri: args.metadata_uri.clone(),
        seller_fee_basis_points: args.seller_fee_basis_points,
        creators: Some(creators),
        collection: None,
        uses: None,
    };
    
    // Create metadata using Anchor SPL wrapper
    anchor_spl::metadata::create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            accounts.metadata_program.to_account_info(),
            anchor_spl::metadata::CreateMetadataAccountsV3 {
                metadata: accounts.metadata_account.to_account_info(),
                mint: accounts.nft_mint.to_account_info(),
                mint_authority: accounts.treasury.to_account_info(),
                payer: accounts.admin.to_account_info(),
                update_authority: accounts.treasury.to_account_info(),
                system_program: accounts.system_program.to_account_info(),
                rent: accounts.rent.to_account_info(),
            },
            &[treasury_seeds]
        ),
        data,
        args.is_mutable,
        true, // update_authority_is_signer
        None  // collection_details
    )?;
    
    // 3. Set NFT as immutable (freeze authority to None)
    anchor_spl::token::set_authority(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            anchor_spl::token::SetAuthority {
                current_authority: accounts.treasury.to_account_info(),
                account_or_mint: accounts.nft_mint.to_account_info(),
            },
            &[treasury_seeds]
        ),
        anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount,
        None
    )?;
    
    // 5. Update escrow status
    accounts.escrow.nft_minted = true;
    
    msg!("NFT minted successfully");
    Ok(())
}

fn calculate_fulfill_rent_cost(rent: &Rent) -> u64 {
    let nft_mint_rent = rent.minimum_balance(82);             // NFT Mint account
    let buyer_nft_ata_rent = rent.minimum_balance(165);       // Buyer's NFT ATA
    
    // Hard-code the exact metadata rent cost based on transaction data
    let nft_metadata_rent = 15_115_600;  // Exact value from transaction logs
    
    msg!("NFT Mint rent: {} lamports", nft_mint_rent);
    msg!("Metadata rent: {} lamports", nft_metadata_rent);
    msg!("Buyer's NFT ATA rent: {} lamports", buyer_nft_ata_rent);
    
    let total = nft_mint_rent + nft_metadata_rent + buyer_nft_ata_rent;
    msg!("Total rent (no master edition): {} lamports", total);
    
    total
}