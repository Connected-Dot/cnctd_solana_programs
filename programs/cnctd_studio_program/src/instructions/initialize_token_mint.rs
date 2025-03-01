use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::{self, MintTo, SetAuthority, Token};
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType as AuthorityType2022;
use anchor_spl::token_2022::{self, SetAuthority as SetAuthority2022, Token2022, MintTo as MintTo2022};
use anchor_spl::associated_token::{self, AssociatedToken};

use crate::errors::CnctdStudioError;
use crate::state::treasury::Treasury;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct InitializeTokenMintArgs {
    pub standard: bool,
}

#[derive(Accounts)]
pub struct InitializeTokenMint<'info> {
    /// CHECK: The mint account for the new token (must be pre-generated and owned by the wallet)
    #[account(mut)]
    pub token_mint: UncheckedAccount<'info>,

    /// CHECK: Treasury PDA that will receive tokens
    #[account(mut)]
    pub treasury_pda: Account<'info, Treasury>,

    /// CHECK: The associated token account (ATA) for the Treasury PDA
    #[account(mut)]
    pub treasury_ata: UncheckedAccount<'info>,

    /// The wallet that initializes and funds the transaction
    #[account(mut)]
    pub signer: Signer<'info>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Token-2022 Program
    pub token_2022_program: Program<'info, Token2022>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_token_mint(ctx: Context<InitializeTokenMint>, data: InitializeTokenMintArgs) -> Result<()> {   
    let standard = data.standard;

    let token_mint = &ctx.accounts.token_mint;
    let treasury_pda = &ctx.accounts.treasury_pda;
    let signer = &ctx.accounts.signer;
    let treasury_ata = &ctx.accounts.treasury_ata; 
    let decimals = 6;

    if !treasury_pda.is_admin(&signer.key()) {
        return Err(CnctdStudioError::Unauthorized.into());
    }

    msg!("Initializing existing mint: {:?}", token_mint.key());

    match standard {
        true => {
            // 1️Initialize the mint (Wallet is the initial authority)
            let cpi_accounts = token::InitializeMint {
                mint: token_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            };

            msg!("CPI Accounts mint: {:?}", cpi_accounts.mint);

            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::initialize_mint(cpi_ctx, decimals, &signer.key(), None)?;

            msg!("Mint Initialized: {:?}", token_mint.key());

            // Create Treasury's ATA
            let cpi_accounts = associated_token::Create {
                payer: signer.to_account_info(),
                associated_token: treasury_ata.to_account_info(),
                authority: treasury_pda.to_account_info(),
                mint: token_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(ctx.accounts.associated_token_program.to_account_info(), cpi_accounts);
            associated_token::create(cpi_ctx)?;
            msg!("Treasury ATA Created: {:?}", treasury_ata);

            // 4️ Mint 1 Billion Tokens to Treasury ATA
            let cpi_accounts = MintTo {
                mint: token_mint.to_account_info(),
                to: treasury_ata.to_account_info(),
                authority: signer.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            let mint_amount = 1_000_000_000u64 * 10u64.pow(decimals as u32);
            token::mint_to(cpi_ctx, mint_amount)?;
            msg!("1 Billion Tokens Minted to Treasury ATA");

            // 5️ Transfer Mint Authority to the Treasury PDA (Instead of the Program)
            let cpi_accounts = SetAuthority {
                current_authority: signer.to_account_info(),
                account_or_mint: token_mint.to_account_info(),
            };

            msg!("Transferring Mint Authority to Treasury PDA: {:?}", treasury_pda.key());

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
            );

            token::set_authority(cpi_ctx, AuthorityType::MintTokens, Some(treasury_pda.key()))?;
            msg!("Mint Authority Transferred to Treasury PDA: {:?}", treasury_pda.key());
        }
        false => {
             // 1️Initialize the mint (Wallet is the initial authority)
            let cpi_accounts = token_2022::InitializeMint {
                mint: token_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            };

            // msg!("CPI Accounts mint: {:?}", cpi_accounts.mint);

            let cpi_ctx = CpiContext::new(ctx.accounts.token_2022_program.to_account_info(), cpi_accounts);
            token_2022::initialize_mint(cpi_ctx, decimals, &signer.key(), None)?;

            // msg!("Mint Initialized: {:?}", token_mint.key());

            // Create Treasury's ATA
            let cpi_accounts = associated_token::Create {
                payer: signer.to_account_info(),
                associated_token: treasury_ata.to_account_info(),
                authority: treasury_pda.to_account_info(),
                mint: token_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_2022_program.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(ctx.accounts.associated_token_program.to_account_info(), cpi_accounts);
            associated_token::create(cpi_ctx)?;
            // msg!("Treasury ATA Created: {:?}", treasury_ata);

            // 4️ Mint 1 Billion Tokens to Treasury ATA
            let cpi_accounts = MintTo2022 {
                mint: token_mint.to_account_info(),
                to: treasury_ata.to_account_info(),
                authority: signer.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(ctx.accounts.token_2022_program.to_account_info(), cpi_accounts);
            let mint_amount = 1_000_000_000u64 * 10u64.pow(decimals as u32);
            token_2022::mint_to(cpi_ctx, mint_amount)?;
            // msg!("1 Billion Tokens Minted to Treasury ATA");

            // 5️ Transfer Mint Authority to the Treasury PDA (Instead of the Program)
            let cpi_accounts = SetAuthority2022 {
                current_authority: signer.to_account_info(),
                account_or_mint: token_mint.to_account_info(),
            };

            msg!("Transferring Mint Authority to Treasury PDA: {:?}", treasury_pda.key());

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_2022_program.to_account_info(),
                cpi_accounts,
            );

            token_2022::set_authority(cpi_ctx, AuthorityType2022::MintTokens, Some(treasury_pda.key()))?;
            msg!("Mint Authority Transferred to Treasury PDA: {:?}", treasury_pda.key());
        }
    }



   

    Ok(())
}