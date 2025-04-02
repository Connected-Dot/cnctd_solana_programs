use anchor_lang::prelude::*;

#[account]
pub struct ReleaseAccess {
    pub release_id: String,
    pub buyer_id: String,
    pub access_type: AccessType,
    pub created_at: i64,
    pub expiration_date: Option<i64>,
    pub access_rights: AccessRights,
    pub clockwork_thread_id: Option<Pubkey>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum AccessType {
    Rental,
    Purchase,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct AccessRights {
    pub can_stream: bool,
    pub can_download: bool,
}