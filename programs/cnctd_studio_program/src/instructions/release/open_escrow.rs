use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    arguments::release::OpenEscrowArgs,
    errors::CnctdStudioError,
    state::{release_escrow::ReleaseEscrow, treasury::Treasury, user_pda::UserPDA},
    utils::UuidFormatting,
};

#[derive(Accounts)]
#[instruction(args: OpenEscrowArgs)]
pub struct OpenEscrow<'info> {
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

    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        space = ReleaseEscrow::space(),
        payer = admin,
        seeds = [
            b"release_escrow",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump
    )]
    pub escrow: Account<'info, ReleaseEscrow>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = usdc_mint,
        associated_token::authority = escrow,
    )]
    pub escrow_usdc_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn open_escrow(ctx: Context<OpenEscrow>, args: OpenEscrowArgs) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    msg!("Creating escrow for buyer: {:?}", ctx.accounts.buyer.key());

    let total_payment = args.treasury_fee + args.payment_splits.iter().map(|split| split.amount).sum::<u64>();

    if escrow.total_amount > 0 {
        msg!("Escrow already funded with {} USDC.", escrow.total_amount);

        ctx.accounts.treasury.reimburse_admin(
            &ctx.accounts.treasury.to_account_info(),
            &ctx.accounts.admin.to_account_info(),
            args.fee_compensation,
            Some(Rent::get()?.minimum_balance(82)),
        )?;
        return Ok(());
    }

    escrow.release_id = args.release_id.clone();
    escrow.buyer_id = args.buyer_id.clone();
    escrow.treasury_fee = args.treasury_fee;
    escrow.payment_splits = args.payment_splits.clone();
    escrow.total_amount = total_payment;
    escrow.fulfilled = false;
    escrow.purchase_date = args.purchase_date;
    escrow.bump = ctx.bumps.escrow;
  
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_usdc_ata.to_account_info(),
                to: ctx.accounts.escrow_usdc_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
            &[&[b"user", args.buyer_id.as_ref(), &[ctx.accounts.buyer.bump]]],
        ),
        total_payment,
    )?;

    msg!("Transferred {} USDC to escrow", total_payment);

    let rent = Rent::get()?;
    let total_rent = calculate_open_escrow_rent_cost(&rent);

    ctx.accounts.treasury.reimburse_admin(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.admin.to_account_info(),
        args.fee_compensation,
        Some(total_rent),
    )?;

    Ok(())
}

fn calculate_open_escrow_rent_cost(rent: &Rent) -> u64 {
    let escrow_account_rent = rent.minimum_balance(ReleaseEscrow::space());
    let escrow_usdc_ata_rent = rent.minimum_balance(165); // standard token account size

    escrow_account_rent + escrow_usdc_ata_rent
}