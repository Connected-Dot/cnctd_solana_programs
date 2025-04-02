use anchor_lang::{prelude::{borsh, Pubkey}, AnchorDeserialize, AnchorSerialize};
use anchor_spl::metadata::mpl_token_metadata::{self, types::DataV2};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

impl Creator {
    pub fn to_metaplex_creator(&self, treasury_key: &Pubkey) -> mpl_token_metadata::types::Creator {
        mpl_token_metadata::types::Creator {
            address: self.address,
            verified: self.address == *treasury_key, // Only mark as verified if it's the treasury
            share: self.share,
        }
    }
    
    pub fn multiple_to_metaplex_creators(
        creators: Vec<Creator>, 
        treasury_key: &Pubkey
    ) -> Vec<mpl_token_metadata::types::Creator> {
        creators.into_iter().map(|creator| {
            creator.to_metaplex_creator(treasury_key)
        }).collect()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

impl Metadata {
    pub fn to_datav2(&self) -> DataV2 {
        let creators = self.creators.clone().map(|creators| {
            creators.into_iter().map(|creator| {
                mpl_token_metadata::types::Creator {
                    address: creator.address,
                    verified: creator.verified,
                    share: creator.share,
                }
            }).collect()
        });
        let collection = self.collection.clone().map(|collection| {
            mpl_token_metadata::types::Collection {
                verified: collection.verified,
                key: collection.key,
            }
        });
        let uses = self.uses.clone().map(|uses| {
            mpl_token_metadata::types::Uses {
                use_method: match uses.use_method {
                    UseMethod::Burn => mpl_token_metadata::types::UseMethod::Burn,
                    UseMethod::Multiple => mpl_token_metadata::types::UseMethod::Multiple,
                    UseMethod::Single => mpl_token_metadata::types::UseMethod::Single,
                },
                remaining: uses.remaining,
                total: uses.total,
            }
        });
        DataV2 {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            uri: self.uri.clone(),
            seller_fee_basis_points: self.seller_fee_basis_points,
            creators,
            collection,
            uses,
        }
    }
}