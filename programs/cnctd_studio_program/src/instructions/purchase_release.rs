use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::{Creator, DataV2}, CreateMetadataAccountsV3
    }, token::{self, mint_to, transfer, MintTo, Token, Transfer}, token_2022::{self, MintTo as MintTo2022, Token2022}
};

use crate::{arguments::purchase_release_args::PurchaseReleaseArgs, errors::CnctdStudioError, state::{treasury::Treasury, user_pda::UserPDA}};

#[derive(Accounts)]
#[instruction(args: PurchaseReleaseArgs)]
pub struct PurchaseRelease<'info> {
    #[account(
        mut, // Must be mutable since it receives and sends SOL
        constraint = treasury.is_admin(&admin.key())
    )]
    pub admin: Signer<'info>,

    // Buyer
    #[account(
        mut,
        seeds = [b"user", args.buyer_id.as_ref()],
        bump
    )]
    pub buyer: Account<'info, UserPDA>,
    
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

    // /// CHECK: This is the treasury's CNCTD token account
    // #[account(mut)]
    // pub treasury_cnctd_ata: AccountInfo<'info>,

    // /// CHECK: This is the treasury's MUS1C token account
    // #[account(mut)]
    // pub treasury_music_ata: AccountInfo<'info>,
    
    // Mints
    /// CHECK: This is the USDC mint
    pub usdc_mint: AccountInfo<'info>,
    
    /// CHECK: This is the MUS1C mint (reward token)
    #[account(mut)] 
    pub music_mint: AccountInfo<'info>,
    
    /// CHECK: This is the CNCTD mint (creator reward token)
    #[account(mut)] 
    pub cnctd_mint: AccountInfo<'info>,
    
    /// CHECK: This is the NFT mint for the album
    #[account(
        mut,
        seeds = [b"release", &args.release_id.as_bytes()[0..std::cmp::min(8, args.release_id.len())], &args.buyer_id.as_bytes()[0..std::cmp::min(8, args.buyer_id.len())]],
        bump,
    )]
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

pub fn purchase_release<'info>(
    ctx: Context<'_, '_, '_, 'info, PurchaseRelease<'info>>,
    args: PurchaseReleaseArgs,
) -> Result<()> {
    let buyer = &ctx.accounts.buyer;
    let treasury = &ctx.accounts.treasury;

    msg!("Purchase album instruction called");
    msg!("Buyer: {:?}", buyer.key());
    msg!("Treasury: {:?}", treasury.key());
    msg!("Release Args: {:?}", args);

    let buyer_seeds = &[b"user" as &[u8], args.buyer_id.as_ref(), &[buyer.bump]];
    let buyer_signer_seeds = &[&buyer_seeds[..]];
    
    // 1. Transfer USDC fee to treasury
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_usdc_ata.clone(),
                to: ctx.accounts.treasury_usdc_ata.clone(),
                authority: buyer.to_account_info(),
            },
            buyer_signer_seeds,
        ),
        args.treasury_fee,
    )?;

    msg!("Transferred {} USDC to treasury", args.treasury_fee);
    
    // 2. Transfer USDC to each artist based on pre-calculated splits
    let remaining_accounts = &ctx.remaining_accounts;

    let bump = ctx.accounts.treasury.bump;
    let seeds: &[&[u8]] = &[b"treasury", &[bump]];
    let treasury_signer_seeds: &[&[&[u8]]] = &[seeds];

    let mut account_index = 0;
    
    for split in args.payment_splits.iter() {
        // For each artist, get their USDC and CNCTD ATAs from remaining accounts
        if account_index + 1 >= remaining_accounts.len() {
            // Not enough accounts - this shouldn't happen if the client is implemented correctly
            return Err(CnctdStudioError::NotEnoughAccounts.into());
        }
        
        // Get references with consistent lifetimes
        let artist_usdc_ata = &remaining_accounts[account_index];
        account_index += 1;
        let artist_cnctd_ata = &remaining_accounts[account_index];
        account_index += 1;
        
        if split.amount == 0 {
            msg!("Skipping zero amount payment to {}", artist_usdc_ata.key());
            continue;
        }
        
        // Transfer USDC to artist
        msg!("Transferring {} USDC to artist: {}", split.amount, artist_usdc_ata.key());
        
        // This is the key fix - create the Transfer struct separately
        let transfer_accounts = Transfer {
            from: ctx.accounts.buyer_usdc_ata.to_account_info(),
            to: artist_usdc_ata.to_account_info(),  // Use to_account_info() on AccountInfo from remaining_accounts
            authority: ctx.accounts.buyer.to_account_info(),
        };
        
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_accounts,
                buyer_signer_seeds,
            ),
            split.amount,
        )?;
        
        // Calculate CNCTD reward (1:1 for this example - you can change the ratio)
        let cnctd_reward = split.amount;
        
        if cnctd_reward > 0 {
            // Mint CNCTD tokens to artist as rewards
            msg!("Minting {} CNCTD tokens to artist: {}", cnctd_reward, artist_cnctd_ata.key());
            
            // Same approach for MintTo
            let cpi_accounts = MintTo2022 {
                mint: ctx.accounts.cnctd_mint.to_account_info(),
                to: artist_cnctd_ata.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            };
            
            token_2022::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_2022_program.to_account_info(),
                    cpi_accounts,
                    treasury_signer_seeds,
                ),
                cnctd_reward,
            )?;
        }
    }
  
    // 3. Mint NFT to buyer
    // Initialize the mint account
    let rent = Rent::get()?;
    let mint_account_info = &ctx.accounts.nft_mint;
    let mint_lamports = rent.minimum_balance(82); // Size of a Mint account
    
    let release_seed = &args.release_id.as_bytes()[0..std::cmp::min(8, args.release_id.len())];
    let buyer_seed = &args.buyer_id.as_bytes()[0..std::cmp::min(8, args.buyer_id.len())];

    let (_, nft_mint_bump) = Pubkey::find_program_address(
        &[b"release", release_seed, buyer_seed],
        &crate::ID
    );
    
    let nft_mint_seeds = &[b"release" as &[u8], release_seed, buyer_seed, &[nft_mint_bump]];
    let nft_mint_signer_seeds = &[&nft_mint_seeds[..]];
    
    // Use PDA signing for account creation
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.admin.key(),
            &mint_account_info.key(),
            mint_lamports,
            82, // Size of a Mint account
            &ctx.accounts.token_program.key(),
        ),
        &[
            ctx.accounts.admin.to_account_info(),
            mint_account_info.clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        nft_mint_signer_seeds,
    )?;
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.nft_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0, // Decimals (0 for NFT)
        &ctx.accounts.treasury.key(), // Mint authority
        Some(&ctx.accounts.treasury.key()), // Freeze authority
    )?;

    // Create the buyer's associated token account for the NFT
    anchor_spl::associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.admin.to_account_info(),
                associated_token: ctx.accounts.buyer_nft_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ),
    )?;
    
    // mint the token
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.buyer_nft_ata.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            },
            treasury_signer_seeds,
        ),
        1, // For NFTs, always mint 1
    )?;
    
    msg!("Minted NFT token");

      // Create metadata for the NFT
    let creators = vec![
        Creator {
            address: treasury.key(),
            verified: true,
            share: 5, // Platform fee share in percentage
        },
    ];

    // Create metadata
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                mint_authority: ctx.accounts.treasury.to_account_info(),
                payer: ctx.accounts.admin.to_account_info(),
                update_authority: treasury.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            treasury_signer_seeds,
        ),
        DataV2 {
            name: args.name,
            symbol: "CNCTD".to_string(),
            uri: args.metadata_uri,
            seller_fee_basis_points: args.seller_fee_basis_points,
            creators: Some(creators),
            collection: None,
            uses: None,
        },
        true, // Is mutable
        true, // Update authority is signer
        None, // Collection details
    )?;
    
    msg!("Created NFT metadata");

    // 4. Issue MUS1C tokens as rewards to buyer
    // Calculate reward amount (e.g., 5% of total spend)
    let total_spend = args.treasury_fee + args.payment_splits.iter().map(|split| split.amount).sum::<u64>();
    let reward_amount = total_spend / 20; // 5% reward
    
    if reward_amount > 0 {
        // Mint MUS1C tokens to buyer as rewards
        let cpi_accounts = MintTo2022 {
            mint: ctx.accounts.music_mint.to_account_info(),
            to: ctx.accounts.buyer_music_ata.to_account_info(),
            authority: ctx.accounts.treasury.to_account_info(),
        };
        
        token_2022::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_2022_program.to_account_info(),
                cpi_accounts,
                treasury_signer_seeds,
            ),
            reward_amount,
        )?;
        
        msg!("Minted {} MUS1C tokens as rewards", reward_amount);
    }

    // 5. Reimburse admin from treasury for any fees or rent
    let treasury_info = ctx.accounts.treasury.to_account_info();
    let admin_info = ctx.accounts.admin.to_account_info();
    let fee_compensation = args.fee_compensation.unwrap_or(0);

    // Calculate rent for NFT token mint (82 bytes)
    let mint_rent = Rent::get()?.minimum_balance(82);

    // Calculate rent for NFT metadata account (approximately 679 bytes)
    let metadata_rent = Rent::get()?.minimum_balance(679);

    // Sum up all rent costs
    let rent_lamports = mint_rent + metadata_rent;

    let total_reimbursement = rent_lamports + fee_compensation;
    
    **treasury_info.try_borrow_mut_lamports()? -= total_reimbursement;
    **admin_info.try_borrow_mut_lamports()? += total_reimbursement;
    
    msg!("Reimbursed admin {} lamports from treasury", total_reimbursement);
    msg!("Final admin balance: {}", ctx.accounts.admin.lamports());
    msg!("Final treasury balance: {}", ctx.accounts.treasury.to_account_info().lamports());

    Ok(())
}
