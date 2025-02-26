use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{self, AssociatedToken, Create}, token::Token};

use crate::state::{program_metadata::ProgramMetadata, treasury::Treasury};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 74, // Space for ProgramMetadata
        seeds = [b"program_metadata"],
        bump
    )]
    pub metadata: Account<'info, ProgramMetadata>,

    #[account(
        init,
        payer = admin,
        space = 8 + 162, // Space for Treasury
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    /// CHECK: This is the associated token account (ATA) for the Treasury PDA
    #[account(mut)]
    pub treasury_ata: UncheckedAccount<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: This is the USDC mint
    pub usdc_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let admin_pubkey = ctx.accounts.admin.key();
    let clock = Clock::get()?;
    let treasury_pda_key = *ctx.accounts.treasury.to_account_info().key;

    // Initialize Program Metadata
    metadata.owner = admin_pubkey;
    metadata.version = 1;
    metadata.treasury_pda = treasury_pda_key;
    metadata.updated_at = clock.unix_timestamp;
    metadata.bump = ctx.bumps.metadata;

    msg!("Initialized Program Metadata: {:?}", metadata.to_account_info().key());

    let treasury = &mut ctx.accounts.treasury;
    treasury.admins = vec![admin_pubkey];
    treasury.bump = ctx.bumps.treasury;
    treasury.version = 1;

    msg!("Initialized Treasury: {:?}", treasury.to_account_info().key());

    // If treasury_ata does not exist, create it
    if ctx.accounts.treasury_ata.to_account_info().owner == &System::id() {
        msg!("Creating Treasury ATA...");
        msg!("Treasury ATA: {:?}", ctx.accounts.treasury_ata.key());
        msg!("USDC Mint: {:?}", ctx.accounts.usdc_mint.key());
        msg!("Authority (Treasury PDA): {:?}", ctx.accounts.treasury.key());
        msg!("Payer: {:?}", ctx.accounts.admin.key());
        msg!("System Program: {:?}", ctx.accounts.system_program.key());
        msg!("Token Program: {:?}", ctx.accounts.token_program.key());
        msg!("Associated Token Program: {:?}", ctx.accounts.associated_token_program.key());
        msg!("Rent: {:?}", ctx.accounts.rent.key());

        let cpi_accounts = Create {
            payer: ctx.accounts.admin.to_account_info(),
            associated_token: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.treasury.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(), // Correct program for ATA creation
            cpi_accounts,
        );
        
        associated_token::create(cpi_ctx)?;
    }

    msg!(
        "Treasury ATA initialized at: {:?}",
        ctx.accounts.treasury_ata.key()
    );

    Ok(())
}
