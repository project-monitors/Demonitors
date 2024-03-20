use anchor_lang::prelude::*;
use anchor_spl::metadata::ID as MPL_TOKEN_METADATA_ID;
use crate::ID;

#[account]
#[derive(Default)]
pub struct MintConfig {
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub mint_bump: u8,
    pub config_bump: u8,
    pub metadata_bump: u8,
    pub authority_bump: u8,
}

impl MintConfig {
    pub const LEN: usize = 32 * 2 + 1 * 4;
    pub const MINT_SEED: &'static [u8] = b"mint";
    pub const MINT_CONFIG_SEED: &'static [u8] = b"mint_config";
    pub const AUTHORITY_SEED: &'static [u8] = b"authority";
    pub const SBT_COLLECTION_SEED: &'static [u8] = b"collection";


    pub fn find_mint(event_config: Option<Pubkey>) -> (Pubkey, u8) {
        match event_config {
            Some(config) => {
                Pubkey::find_program_address(&[Self::MINT_SEED, config.as_ref()], &ID)
            }
            None => {
                Pubkey::find_program_address(&[Self::MINT_SEED], &ID)
            }
        }
    }

    pub fn find_mint_config(event_config: Option<Pubkey>) -> (Pubkey, u8) {
        match event_config {
            Some(config) => {
                Pubkey::find_program_address(&[Self::MINT_CONFIG_SEED, config.as_ref()], &ID)
            }
            None => {
                Pubkey::find_program_address(&[Self::MINT_CONFIG_SEED], &ID)
            }
        }
    }

    pub fn find_authority(mint: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::AUTHORITY_SEED, mint.as_ref()], &ID)
    }

    pub fn find_metadata(mint: Pubkey) -> Result<(Pubkey, u8)> {
        let metadata_program_id = Pubkey::try_from_slice(MPL_TOKEN_METADATA_ID.as_ref())?;
        Ok(Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                metadata_program_id.as_ref(),
                mint.as_ref(),
            ],
            &metadata_program_id,
        ))
    }

    pub fn find_master_edition(mint: Pubkey) -> Result<(Pubkey, u8)> {
        let metadata_program_id = Pubkey::try_from_slice(MPL_TOKEN_METADATA_ID.as_ref())?;
        Ok(
            Pubkey::find_program_address(
                &[
                    "metadata".as_bytes(),
                    metadata_program_id.as_ref(),
                    mint.as_ref(),
                    "edition".as_bytes(),
                ],
                &metadata_program_id
            )
        )
    }
}