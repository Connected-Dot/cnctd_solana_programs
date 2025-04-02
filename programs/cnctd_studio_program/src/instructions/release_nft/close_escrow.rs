use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount, Transfer};
use crate::{arguments::release_nft::CloseEscrowArgs, errors::CnctdStudioError, state::{release_escrow::ReleaseEscrow, treasury::Treasury}, utils::UuidFormatting};

#[derive(Accounts)]
#[instruction(args: CloseEscrowArgs)]
pub struct CloseEscrow<'info> {
    #[account(
        mut,
        seeds = [
            b"release_escrow",
            args.release_id.to_7_byte_seed().as_ref(),
            args.buyer_id.to_7_byte_seed().as_ref(),
        ],
        bump,
        close = treasury
    )]
    pub escrow: Account<'info, ReleaseEscrow>,

    #[account(mut)]
    pub escrow_usdc_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub treasury_usdc_ata: Account<'info, TokenAccount>,

    #[account(constraint = treasury.is_admin(&admin.key()))]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


pub fn close_escrow(ctx: Context<CloseEscrow>, args: CloseEscrowArgs) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    msg!("Closing escrow account {:?}", escrow.key());
    msg!("Closing escrow USDC account {:?}", ctx.accounts.escrow_usdc_ata.key());

    let release_seed = args.release_id.to_7_byte_seed();
    let buyer_seed = args.buyer_id.to_7_byte_seed();

    let escrow_seeds = &[
        b"release_escrow" as &[u8],
        &release_seed,
        &buyer_seed,
        &[escrow.bump],
    ];
    let signer_seeds = &[&escrow_seeds[..]];

    let amount = ctx.accounts.escrow_usdc_ata.amount;
    msg!("Token account balance: {}", amount);

    if amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow_usdc_ata.to_account_info(),
                    to: ctx.accounts.treasury_usdc_ata.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        ).map_err(|_| error!(CnctdStudioError::TransferFailed))?;

        msg!("Successfully transferred {} tokens to treasury", amount);
    } else {
        msg!("Token account has zero balance, no transfer needed");
    }

    token::close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.escrow_usdc_ata.to_account_info(),
                destination: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            signer_seeds,
        ),
    ).map_err(|_| error!(CnctdStudioError::CloseAccountFailed))?;

    msg!("Successfully closed token account");
    Ok(())
}
