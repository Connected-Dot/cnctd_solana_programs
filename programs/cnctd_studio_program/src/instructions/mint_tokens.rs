use anchor_lang::prelude::*;
use anchor_spl::{token::{self, MintTo, Token}, token_2022::{self, MintTo as MintTo2022, Token2022}};

use crate::{errors::CnctdStudioError, state::treasury::Treasury};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct MintTokensArgs {
    pub amount: u64,
    pub standard: bool,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    /// CHECK: The mint account for the token
    #[account(mut)]
    pub mint: UncheckedAccount<'info>, // Token-2022 Mint Account

    /// CHECK: The destination account for the tokens
    #[account(mut)]
    pub destination: UncheckedAccount<'info>, // Receiver's token account (ATA)

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>, // PDA that is the mint authority

    #[account(signer)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>, // Token program    

    pub token_program_2022: Program<'info, Token2022>, // Token-2022 program
}

pub fn mint_tokens(ctx: Context<MintTokens>, data: MintTokensArgs) -> Result<()> {
    let amount = data.amount;
    let standard = data.standard;

    if !ctx.accounts.treasury.is_admin(&ctx.accounts.signer.key()) {
        return Err(CnctdStudioError::Unauthorized.into());
    }
    let bump = ctx.accounts.treasury.bump;
    let seeds: &[&[u8]] = &[b"treasury", &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    msg!("Minting {} tokens to {:?}", amount, ctx.accounts.destination.key());

    match standard {
        true => {
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            };
        
            token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    cpi_accounts,
                    signer_seeds,
                ),
                amount,
            )?;
        
            msg!("Successfully minted {} tokens to {:?}", amount, ctx.accounts.destination.key());
        }
        false => {
            let cpi_accounts = MintTo2022 {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            };
        
            token_2022::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program_2022.to_account_info(),
                    cpi_accounts,
                    signer_seeds,
                ),
                amount,
            )?;
        
            msg!("Successfully minted {} tokens to {:?}", amount, ctx.accounts.destination.key());
        }   
    }

    
    Ok(())
}
