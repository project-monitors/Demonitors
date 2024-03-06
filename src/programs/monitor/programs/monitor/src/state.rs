use anchor_lang::prelude::*;
use crate::error::ErrorCode;


#[account]
pub struct OracleConfig {
    pub name: String,
    pub description: String,
    pub total_phases: u8,
    pub authority_pubkeys: Vec<Pubkey>,
    pub admin: Pubkey,
    pub bump: u8,
}

impl OracleConfig {
    pub fn has_authority(&self, pubkey: &Pubkey) -> bool {
        self.authority_pubkeys.contains(pubkey)
    }

    pub fn add_authority(&mut self, user: Pubkey, pubkey: Pubkey) -> Result<()> {
        require_keys_eq!(user.key(), self.admin, ErrorCode::Unauthorized);
        if !self.has_authority(&pubkey) {
            require!(self.authority_pubkeys.len() <= 4, ErrorCode::TooManyAuthorities);
            self.authority_pubkeys.push(pubkey);
        }
        Ok(())
    }

    pub fn remove_authority(&mut self, user: Pubkey, pubkey: Pubkey) -> Result<()> {
        require_keys_eq!(user.key(), self.admin, ErrorCode::Unauthorized);
        require!(self.has_authority(&pubkey), ErrorCode::AuthorityNotFound);
        if let Some(index) = self.authority_pubkeys.iter().position(|x| *x == pubkey) {
            self.authority_pubkeys.remove(index);
        }
        Ok(())
    }

}

#[account]
pub struct OracleData {
    pub config: Pubkey,
    pub phase: u8,
    pub raw_data: u64,
    pub decimals: u8,
    pub timestamp: u64,
    pub bump: u8,
}

