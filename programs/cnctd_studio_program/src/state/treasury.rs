use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub admins: Vec<Pubkey>, // Multiple admins
    pub bump: u8, 
    pub version: u8,
}

impl Treasury {
    pub fn is_admin(&self, pubkey: &Pubkey) -> bool {
        self.admins.iter().any(|admin| admin == pubkey)
    }
    pub fn reimburse_admin<'info>(
        &self,
        treasury_info: &AccountInfo<'info>,
        admin_info: &AccountInfo<'info>,
        fee_compensation: Option<u64>,
        additional_rent: Option<u64>,
    ) -> Result<()> {
        let fee_compensation = fee_compensation.unwrap_or(0);
        let additional_rent = additional_rent.unwrap_or(0);
        
        let total_reimbursement = fee_compensation + additional_rent;
    
        msg!("Starting reimbursement process:");
        msg!(" - Fee compensation: {} lamports", fee_compensation);
        msg!(" - Additional rent reimbursement: {} lamports", additional_rent);
        msg!(" - Total reimbursement amount: {} lamports", total_reimbursement);
        msg!("Treasury balance before reimbursement: {} lamports", treasury_info.lamports());
        msg!("Admin balance before reimbursement: {} lamports", admin_info.lamports());
    
        **treasury_info.try_borrow_mut_lamports()? -= total_reimbursement;
        **admin_info.try_borrow_mut_lamports()? += total_reimbursement;
    
        msg!("Treasury balance after reimbursement: {} lamports", treasury_info.lamports());
        msg!("Admin balance after reimbursement: {} lamports", admin_info.lamports());
        msg!("Reimbursement completed successfully.");
    
        Ok(())
    }
    
}

