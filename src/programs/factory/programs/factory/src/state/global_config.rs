use anchor_lang::prelude::*;
use crate::ID;

/// The [GlobalConfig] account.
#[account]
#[derive(Default)]
pub struct GlobalConfig {
    /// The admin of the factory program.
    pub admin: Pubkey,
    /// The off-chain supervisor for vision mining.
    pub vision_mining_admin: Pubkey,
    pub vision_mining_pda: Pubkey,
    pub event_mining_pda: Pubkey,
    pub stake_mining_pda: Pubkey,
    pub global_config_bump: u8,
    pub vision_mining_bump: u8,
    pub event_mining_bump: u8,
    pub stake_mining_bump: u8,
}

impl GlobalConfig {
    pub const LEN: usize = 32 * 5 + 1 * 4;

    pub const GLOBAL_CONFIG_SEED: &'static [u8] = b"global_config";
    pub const VISION_MINING_SEED: &'static [u8] = b"ft_vision_mining_pda";
    pub const EVENT_MINING_SEED: &'static [u8] = b"ft_event_mining_pda";
    pub const STAKE_MINING_SEED: &'static [u8] = b"ft_stake_mining_pda";

    pub fn find_global_config_account() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::GLOBAL_CONFIG_SEED], &ID)
    }

    pub fn find_vision_mining_authority() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::VISION_MINING_SEED], &ID)
    }

    pub fn find_event_mining_authority() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::EVENT_MINING_SEED], &ID)
    }

    pub fn find_stake_mining_authority() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::STAKE_MINING_SEED], &ID)
    }

}