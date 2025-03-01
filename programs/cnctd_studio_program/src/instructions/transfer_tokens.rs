use anchor_lang::prelude::{borsh::{BorshDeserialize, BorshSerialize}, *};
use anchor_spl::{associated_token::{self, AssociatedToken}, token::{self, Token, TokenAccount, Transfer}, token_2022::{self, Token2022, TransferChecked as Transfer2022}};

use crate::{errors::CnctdStudioError, state::treasury::Treasury};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct TransferTokensArgs {
    pub amount: u64,
    pub standard: bool,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    /// CHECK: Token mint of the specific token being transferred
    pub token_mint: UncheckedAccount<'info>,

    /// Treasury PDA that holds the tokens
    #[account(mut,
        seeds = [b"treasury"],
        bump = treasury_pda.bump,
    )]
    pub treasury_pda: Account<'info, Treasury>,

    /// CHECK: The associated token account (ATA) for the Treasury PDA
    #[account(mut)]
    pub treasury_ata: UncheckedAccount<'info>,

    /// CHECK: The recipient who should receive the tokens
    #[account(mut)]
    pub recipient_pda: UncheckedAccount<'info>,

    /// CHECK: Recipient's associated token account (ATA) for receiving the tokens
    #[account(mut)]
    pub recipient_ata: UncheckedAccount<'info>,

    /// CHECK: Admin of the treasury
    #[account(mut)]
    pub admin: UncheckedAccount<'info>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Token-2022 Program
    pub token_2022_program: Program<'info, Token2022>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens(ctx: Context<TransferTokens>, data: TransferTokensArgs) -> Result<()> {
    let treasury_pda = &ctx.accounts.treasury_pda;
    let treasury_ata = &ctx.accounts.treasury_ata;
    let admin = &ctx.accounts.admin;
    let recipient_pda = &ctx.accounts.recipient_pda;
    let recipient_ata = &ctx.accounts.recipient_ata;
    let token_mint = &ctx.accounts.token_mint;

    let amount = data.amount;
    let standard = data.standard;

    msg!("Standard: {:?}", standard);

    let token_program = match standard {
        true => ctx.accounts.token_program.to_account_info(),
        false => ctx.accounts.token_2022_program.to_account_info(),
    };

    // Ensure the signer is authorized to send tokens
    if !treasury_pda.is_admin(&admin.key()) {
        return Err(CnctdStudioError::Unauthorized.into());
    }

    msg!("Transferring {} tokens from Treasury to recipient", amount);

    if recipient_ata.to_account_info().owner == &System::id() {
        msg!("Recipient ATA: {:?} does not exist. Creating ATA...", recipient_ata.key());

        // msg!("Signer: {:?}", admin.key());
        // msg!("Recipient PDA: {:?}", recipient_pda.key());
        // msg!("Recipient ATA: {:?}", recipient_ata.key());
        // msg!("Token Mint: {:?}", token_mint.key());
        // msg!("System Program: {:?}", ctx.accounts.system_program.key());
        msg!("Token Program: {:?}", token_program.key());
        // msg!("Associated Token Program: {:?}", ctx.accounts.associated_token_program.key());
        let cpi_accounts = associated_token::Create {
            payer: admin.to_account_info(),
            associated_token: recipient_ata.to_account_info(),
            authority: recipient_pda.to_account_info(),
            mint: token_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: token_program.to_account_info(),
        };
        

        let cpi_ctx = CpiContext::new(ctx.accounts.associated_token_program.to_account_info(), cpi_accounts);
        associated_token::create(cpi_ctx)?;

        msg!("Recipient ATA Created: {:?}", recipient_ata.key());
    }

    match standard {
        true => {
            let cpi_accounts = Transfer {
                from: treasury_ata.to_account_info(),
                to: recipient_ata.to_account_info(),
                authority: treasury_pda.to_account_info(),
            };
        
            let bump = ctx.accounts.treasury_pda.bump;
            let seeds: &[&[u8]] = &[b"treasury", &[bump]];
            let signer_seeds: &[&[&[u8]]] = &[seeds];
        
            msg!("Transferring tokens...");
            msg!("Treasury PDA: {:?}", ctx.accounts.treasury_pda.key());
            msg!("Treasury ATA: {:?}", ctx.accounts.treasury_ata.key());
            msg!("Treasury ATA Owner: {:?}", ctx.accounts.treasury_ata.owner);
            msg!("Recipient ATA: {:?}", ctx.accounts.recipient_ata.key());
            msg!("Recipient ATA Owner: {:?}", ctx.accounts.recipient_ata.owner);
            msg!("Recipient ATA Data: {:?}", recipient_ata.to_account_info().data_len());
            msg!("Token Mint: {:?}", ctx.accounts.token_mint.key());
            msg!("Token Program: {:?}", token_program.key());
        
        
            let cpi_ctx = CpiContext::new_with_signer(
                token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
        
            token::transfer(cpi_ctx, amount)?;
        
        }
        false => {
            let cpi_accounts = Transfer2022 {
                from: treasury_ata.to_account_info(),
                mint: token_mint.to_account_info(),
                to: recipient_ata.to_account_info(),
                authority: treasury_pda.to_account_info(),
            };
        
            let bump = ctx.accounts.treasury_pda.bump;
            let seeds: &[&[u8]] = &[b"treasury", &[bump]];
            let signer_seeds: &[&[&[u8]]] = &[seeds];
        
            msg!("Transferring tokens...");
            msg!("Treasury PDA: {:?}", ctx.accounts.treasury_pda.key());
            msg!("Treasury ATA: {:?}", ctx.accounts.treasury_ata.key());
            msg!("Treasury ATA Owner: {:?}", ctx.accounts.treasury_ata.owner);
            msg!("Recipient ATA: {:?}", ctx.accounts.recipient_ata.key());
            msg!("Recipient ATA Owner: {:?}", ctx.accounts.recipient_ata.owner);
            msg!("Recipient ATA Data: {:?}", recipient_ata.to_account_info().data_len());
            msg!("Token Mint: {:?}", ctx.accounts.token_mint.key());
            msg!("Token Program: {:?}", token_program.key());
        
        
            let cpi_ctx = CpiContext::new_with_signer(
                token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
        
            token_2022::transfer_checked(cpi_ctx, amount, 6)?;
        
        }
    }
    
   
    msg!("Transfer successful!");

    Ok(())
}
