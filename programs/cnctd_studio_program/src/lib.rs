use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;
pub mod requests;

use instructions::mint_album::*;

declare_id!("4X4ghmVDL7B29vNns8qizXsMUcsE8TZi1EvPewLsPkrN");

#[program]
pub mod cnctd_studio_program {
    use crate::requests::contract_request::ContractRequest;

    use super::*;

    pub fn mint_album(ctx: Context<MintAlbum>, contract_request: ContractRequest) -> Result<()> {
        msg!("Minting Album...");

        msg!("Album ID: {}", contract_request.album_id);
        msg!("Price: {}", contract_request.price);
        msg!("Credits:");
        for credit in &contract_request.credits {
            msg!("  - Artist ID: {}", credit.artist_id);
            msg!("    Writing: {}", credit.writing);
            msg!("    Performance: {}", credit.performance);
        }
   
        Ok(())
    }
}
