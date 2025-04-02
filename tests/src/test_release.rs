#[test]
fn test_payment_limits() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "cnctd_studio_program",
        program_id,
        processor!(process_instruction),
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().unwrap();
    
    // Create the test data
    let treasury_keypair = Keypair::new();
    let buyer_keypair = Keypair::new();
    let admin_keypair = Keypair::new();
    
    // Set up treasury account
    let treasury_seeds = &[b"treasury"];
    let (treasury_pda, treasury_bump) = Pubkey::find_program_address(treasury_seeds, &program_id);
    
    // Create the test data
    let release_id = "test-release-123".to_string();
    let buyer_id = "test-buyer-456".to_string();
    
    // Create USDC mint
    let usdc_mint_keypair = Keypair::new();
    
    // Set up escrow PDA
    let release_seed = [0; 7]; // Simplified for testing
    let buyer_seed = [0; 7];   // Simplified for testing
    let escrow_seeds = &[
        b"release_escrow",
        &release_seed,
        &buyer_seed,
    ];
    let (escrow_pda, escrow_bump) = Pubkey::find_program_address(escrow_seeds, &program_id);
    
    // Create test payment splits (adjust the number to test limits)
    let num_splits = 8; // Try with different values to find the limit
    let mut payment_splits = Vec::new();
    let mut remaining_accounts = Vec::new();
    
    // Create SPL token mints, accounts and payment splits
    for i in 0..num_splits {
        let recipient_keypair = Keypair::new();
        let recipient_usdc_ata = get_associated_token_address(
            &recipient_keypair.pubkey(),
            &usdc_mint_keypair.pubkey(),
        );
        
        // Create account in test environment
        create_token_account(
            &mut banks_client,
            &payer,
            &recent_blockhash,
            &recipient_usdc_ata,
            &usdc_mint_keypair.pubkey(),
            &recipient_keypair.pubkey(),
        ).unwrap();
        
        // Add to payment splits
        payment_splits.push(PaymentSplit {
            recipient_usdc_ata,
            recipient_cnctd_ata: Pubkey::new_unique(),  // Not used in test
            amount: 1_000_000,  // 1 USDC
        });
        
        // Add to remaining accounts
        remaining_accounts.push(AccountMeta::new(recipient_usdc_ata, false));
    }
    
    // Set up escrow account with test data
    let escrow = ReleaseEscrow {
        release_id: release_id.clone(),
        buyer_id: buyer_id.clone(),
        treasury_fee: 500_000,  // 0.5 USDC
        payment_splits: payment_splits.clone(),
        total_amount: 8_500_000,  // 8.5 USDC (8 recipients + treasury fee)
        payments_fulfilled: false,
        nft_minted: false,
        rewards_paid: false,
        fulfilled: false,
        purchase_date: 1678900000,  // Some timestamp
        bump: escrow_bump,
    };
    
    // Create escrow account in test environment
    create_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &escrow_pda,
        &escrow,
        ReleaseEscrow::space() as u64,
        program_id,
    ).unwrap();
    
    // Fund escrow with USDC
    let escrow_usdc_ata = get_associated_token_address(
        &escrow_pda,
        &usdc_mint_keypair.pubkey(),
    );
    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &escrow_usdc_ata,
        &usdc_mint_keypair.pubkey(),
        &escrow_pda,
    ).unwrap();
    mint_tokens(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &usdc_mint_keypair.pubkey(),
        &escrow_usdc_ata,
        &payer,
        escrow.total_amount,
    ).unwrap();
    
    // Create treasury USDC ATA
    let treasury_usdc_ata = get_associated_token_address(
        &treasury_pda,
        &usdc_mint_keypair.pubkey(),
    );
    create_token_account(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        &treasury_usdc_ata,
        &usdc_mint_keypair.pubkey(),
        &treasury_pda,
    ).unwrap();
    
    // Create the buyer's NFT account for completion of the test
    let nft_mint_keypair = Keypair::new();
    let buyer_pda_seeds = &[b"user", buyer_id.as_bytes()];
    let (buyer_pda, _) = Pubkey::find_program_address(buyer_pda_seeds, &program_id);
    let buyer_nft_ata = get_associated_token_address(
        &buyer_pda,
        &nft_mint_keypair.pubkey(),
    );
    
    // Metadata account
    let metadata_seeds = &[
        b"metadata",
        &mpl_token_metadata::ID.to_bytes(),
        &nft_mint_keypair.pubkey().to_bytes(),
    ];
    let (metadata_account, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    
    // Create instruction arguments
    let args = FulfillReleaseArgs {
        release_id,
        buyer_id,
        name: "Test Release".to_string(),
        symbol: "TEST".to_string(),
        metadata_uri: "https://example.com/metadata.json".to_string(),
        is_mutable: false,
        seller_fee_basis_points: 500,
        creators: Vec::new(), // Empty for payment testing
        fee_compensation: Some(10000),
    };
    
    // Build transaction
    let accounts = vec![
        AccountMeta::new(admin_keypair.pubkey(), true),
        AccountMeta::new(buyer_pda, false),
        AccountMeta::new(treasury_pda, false),
        AccountMeta::new(treasury_usdc_ata, false),
        AccountMeta::new_readonly(usdc_mint_keypair.pubkey(), false),
        AccountMeta::new(escrow_pda, false),
        AccountMeta::new(escrow_usdc_ata, false),
        AccountMeta::new(nft_mint_keypair.pubkey(), false),
        AccountMeta::new(buyer_nft_ata, false),
        AccountMeta::new(metadata_account, false),
        AccountMeta::new_readonly(mpl_token_metadata::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(sysvar::rent::ID, false),
    ];
    
    // Add remaining accounts
    let mut all_accounts = accounts.clone();
    all_accounts.extend(remaining_accounts);
    
    // Build instruction
    let instruction = Instruction {
        program_id,
        accounts: all_accounts,
        data: anchor_lang::InstructionData::data(&crate::instruction::FulfillRelease::new(args)),
    };
    
    // Process transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    
    let result = banks_client.process_transaction(transaction).await;
    
    // Analyze result
    match result {
        Ok(_) => println!("Success with {} payment splits", num_splits),
        Err(e) => {
            println!("Error with {} payment splits: {:?}", num_splits, e);
            // Analyze error - if it's a compute budget issue, we hit the limit
            if e.to_string().contains("compute budget exceeded") {
                println!("Compute budget exceeded - max safe payment splits is < {}", num_splits);
            }
        }
    }
}