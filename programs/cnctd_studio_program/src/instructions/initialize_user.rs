use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction}};
use anchor_spl::{associated_token::{create, AssociatedToken}, token::Token, token_2022::Token2022};
use crate::{arguments::initialize_user_args::InitializeUserArgs, state::{treasury::Treasury, user_pda::UserPDA}};

#[derive(Accounts)]
#[instruction(args: InitializeUserArgs)]
pub struct InitializeUser<'info> {
    /// CHECK: The user PDA being initialized
    #[account(mut)]
    pub user_pda: UncheckedAccount<'info>,
    
    // Treasury - source of funds
    #[account(
        mut, // Must be mutable since it sends SOL
        seeds = [b"treasury"],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    
    // Admin - temporary intermediary payer
    #[account(
        mut, // Must be mutable since it receives and sends SOL
        constraint = treasury.is_admin(&admin.key())
    )]
    pub admin: Signer<'info>,
    
    /// CHECK: Trasury USDC ATA
    #[account(mut)]
    pub treasury_usdc_ata: UncheckedAccount<'info>,
    
    /// CHECK: USDC token mint
    pub usdc_mint: UncheckedAccount<'info>,
    
    /// CHECK: USDC ATA for user
    #[account(mut)]
    pub usdc_ata: UncheckedAccount<'info>,
    
    // CNCTD accounts (Token-2022 program)
    /// CHECK: CNCTD token mint
    pub cnctd_mint: UncheckedAccount<'info>,
    
    /// CHECK: CNCTD ATA for user
    #[account(mut)]
    pub cnctd_ata: UncheckedAccount<'info>,
    
    // MUSIC accounts (Token-2022 program)
    /// CHECK: MUSIC token mint
    pub music_mint: UncheckedAccount<'info>,
    
    /// CHECK: MUSIC ATA for user
    #[account(mut)]
    pub music_ata: UncheckedAccount<'info>,
    
    // Programs
    pub token_program: Program<'info, Token>,
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


pub fn initialize_user(ctx: Context<InitializeUser>, args: InitializeUserArgs) -> Result<()> {
    msg!("Treasury balance: {}", ctx.accounts.treasury.to_account_info().lamports());
    msg!("Treasury key: {}", ctx.accounts.treasury.key());
    msg!("User PDA key: {}", ctx.accounts.user_pda.key());
    msg!("Admin key: {}", ctx.accounts.admin.key());
    
    
    
    // Create the user PDA account and initialize it
    create_user_pda(&ctx, &args)?;
    
    msg!("UserPDA initialized successfully");
    Ok(())
}

fn create_user_pda(ctx: &Context<InitializeUser>, args: &InitializeUserArgs) -> Result<()> {
    let user_id = &args.user_id;
    let fee_compensation = args.fee_compensation.unwrap_or(0);
    
    // Calculate user PDA and bump
    let (_, user_bump) = Pubkey::find_program_address(
        &[b"user", user_id.as_bytes()],
        &crate::ID
    );
    
    // Calculate the required lamports for rent exemption
    let space = 8 + std::mem::size_of::<UserPDA>();
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(space);
    
    // Log info
    msg!("Rent required for user PDA: {}", rent_lamports);
    msg!("Admin balance before: {}", ctx.accounts.admin.lamports());
    msg!("Treasury balance before: {}", ctx.accounts.treasury.to_account_info().lamports());
    
    // Create user PDA using invoke_signed
    let user_seeds = &[b"user" as &[u8], user_id.as_bytes(), &[user_bump]];
    let user_signer_seeds = &[&user_seeds[..]];
    
    // Create the account directly from the admin
    invoke_signed(
        &system_instruction::create_account(
            &ctx.accounts.admin.key(),  // Admin is the payer 
            &ctx.accounts.user_pda.key(),
            rent_lamports,
            space as u64,
            &crate::ID,
        ),
        &[
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.user_pda.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        user_signer_seeds,
    )?;
    
    msg!("User PDA account created");

    if ctx.accounts.usdc_ata.data_is_empty() {
        // Create USDC ATA (regular Token program)
        msg!("Creating USDC ATA for user");
        create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.admin.to_account_info(),
                associated_token: ctx.accounts.usdc_ata.to_account_info(),
                authority: ctx.accounts.user_pda.to_account_info(),
                mint: ctx.accounts.usdc_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;
        
        msg!("USDC ATA created: {}", ctx.accounts.usdc_ata.key());
    }
    
   
    if ctx.accounts.cnctd_ata.data_is_empty() {
        // Create CNCTD ATA (Token-2022 program)
        msg!("Creating CNCTD ATA for user");
        create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.admin.to_account_info(),
                associated_token: ctx.accounts.cnctd_ata.to_account_info(),
                authority: ctx.accounts.user_pda.to_account_info(),
                mint: ctx.accounts.cnctd_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_2022_program.to_account_info(),
            },
        ))?;
        
        msg!("CNCTD ATA created: {}", ctx.accounts.cnctd_ata.key());
    }
    
    if ctx.accounts.music_ata.data_is_empty() {
        // Create MUSIC ATA (Token-2022 program)
        msg!("Creating MUSIC ATA for user");
        create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.admin.to_account_info(),
                associated_token: ctx.accounts.music_ata.to_account_info(),
                authority: ctx.accounts.user_pda.to_account_info(),
                mint: ctx.accounts.music_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_2022_program.to_account_info(),
            },
        ))?;
        
        msg!("MUSIC ATA created: {}", ctx.accounts.music_ata.key());
    }
    
    // Deposit USDC if requested
    if let Some(usdc_amount) = args.usdc_deposit_amount {
        if usdc_amount > 0 {
            msg!("Depositing {} USDC to user", usdc_amount);
            
            let bump = ctx.accounts.treasury.bump;
            let seeds: &[&[u8]] = &[b"treasury", &[bump]];
            let treasury_signer_seeds: &[&[&[u8]]] = &[seeds];
            
            // Transfer USDC from treasury to user
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.treasury_usdc_ata.to_account_info(),
                        to: ctx.accounts.usdc_ata.to_account_info(),
                        authority: ctx.accounts.treasury.to_account_info(),
                    },
                    treasury_signer_seeds,
                ),
                usdc_amount,
            )?;
            
            msg!("USDC deposited successfully");
        }
    }
    
    // Now transfer SOL from treasury to admin to reimburse them
    let treasury_info = ctx.accounts.treasury.to_account_info();
    let admin_info = ctx.accounts.admin.to_account_info();

    let total_reimbursement = rent_lamports + fee_compensation;
    
    **treasury_info.try_borrow_mut_lamports()? -= total_reimbursement;
    **admin_info.try_borrow_mut_lamports()? += total_reimbursement;
    
    msg!("Reimbursed admin {} lamports from treasury", total_reimbursement);
    msg!("Final admin balance: {}", ctx.accounts.admin.lamports());
    msg!("Final treasury balance: {}", ctx.accounts.treasury.to_account_info().lamports());
    
    // Initialize the UserPDA fields
    let mut user_pda: UserPDA = UserPDA::try_deserialize_unchecked(
        &mut &ctx.accounts.user_pda.data.borrow()[..]
    )?;
    
    // Set the fields with actual ATA pubkeys
    user_pda.admin = ctx.accounts.treasury.key();
    user_pda.auth = None;
    user_pda.usdc_ata = ctx.accounts.usdc_ata.key();
    user_pda.usdc_cust = None;  // No custom ATA initially
    user_pda.cnctd_ata = ctx.accounts.cnctd_ata.key();
    user_pda.cnctd_cust = None;  // No custom ATA initially
    user_pda.music_ata = ctx.accounts.music_ata.key();
    user_pda.music_cust = None;  // No custom ATA initially
    user_pda.fees_waived = 0;
    user_pda.waived_count = 0;
    user_pda.bump = user_bump;
    
    // Serialize back to the account
    user_pda.try_serialize(&mut &mut ctx.accounts.user_pda.data.borrow_mut()[..])?;
    
    msg!("UserPDA initialized successfully");
    Ok(())
}