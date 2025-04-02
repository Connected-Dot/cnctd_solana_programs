use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_2022::Token2022};
use crate::{arguments::initialize_band_args::InitializeBandArgs, state::{treasury::Treasury, band_pda::BandPDA}};

#[derive(Accounts)]
#[instruction(args: InitializeBandArgs)]
pub struct InitializeBand<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + std::mem::size_of::<BandPDA>(),
        seeds = [b"band", args.band_id.as_bytes()],
        bump
    )]
    pub band_pda: Account<'info, BandPDA>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        constraint = treasury.is_admin(&admin.key())
    )]
    pub admin: Signer<'info>,
    
    pub usdc_mint: Account<'info, anchor_spl::token::Mint>,
    
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = usdc_mint,
        associated_token::authority = band_pda,
    )]
    pub usdc_ata: Account<'info, anchor_spl::token::TokenAccount>,
    
    /// CHECK: This is a Token-2022 mint account
    pub cnctd_mint: UncheckedAccount<'info>,
    
    /// CHECK: This is a Token-2022 token account
    #[account(mut)]
    pub cnctd_ata: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_band(ctx: Context<InitializeBand>, args: InitializeBandArgs) -> Result<()> {
    msg!("Initializing band: {}", args.band_id);

    // Create CNCTD ATA manually since we're using UncheckedAccount
    if ctx.accounts.cnctd_ata.data_is_empty() {
        msg!("Creating CNCTD ATA for band");
        anchor_spl::associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.admin.to_account_info(),
                    associated_token: ctx.accounts.cnctd_ata.to_account_info(),
                    authority: ctx.accounts.band_pda.to_account_info(),
                    mint: ctx.accounts.cnctd_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_2022_program.to_account_info(),
                },
            ),
        )?;
        
        msg!("CNCTD ATA created: {}", ctx.accounts.cnctd_ata.key());
    }

    
    // Set the initialized fields
    let band = &mut ctx.accounts.band_pda;
    band.admin = ctx.accounts.treasury.key();
    band.usdc_ata = ctx.accounts.usdc_ata.key();
    band.cnctd_ata = ctx.accounts.cnctd_ata.key();
    band.fees_waived = 0;
    band.waived_count = 0;
    band.bump = ctx.bumps.band_pda;
    
    // Reimburse the admin for any fees incurred
    let space = 8 + std::mem::size_of::<BandPDA>();
    let rent_lamports = Rent::get()?.minimum_balance(space);

    ctx.accounts.treasury.reimburse_admin(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.admin.to_account_info(),
        args.fee_compensation,
        Some(rent_lamports),
    )?;
    
    msg!("Band PDA initialized successfully");

    Ok(())
}