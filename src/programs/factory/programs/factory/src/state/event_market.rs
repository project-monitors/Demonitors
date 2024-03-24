use anchor_lang::prelude::*;
use crate::ID;


#[account]
pub struct EventMarket {
    pub event_config: Pubkey,
    pub open_raw_data: Option<u64>,
    pub open_phase: Option<u8>,
    pub option: u8,
    pub result: u8,
    pub prize: u64,
    pub open_slot: u64,
    pub open_ts: u64,
    pub close_ts: u64,
    pub expiry_ts: u64,
    pub is_opened: bool,
    pub bump: u8,
}

impl EventMarket {

    pub const LEN: usize = 32 + 9 + 2 + 1 * 2 + 8 * 5 + 1 * 2;
    pub const EVENT_MARKET_SEED: &'static [u8] = b"event_market";

    pub fn find_event_market_account(oracle_config_pubkey: Pubkey, open_ts: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::EVENT_MARKET_SEED,
                oracle_config_pubkey.as_ref(),
                &open_ts.to_be_bytes()[..]],
            &ID
        )
    }
}