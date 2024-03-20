use anchor_lang::prelude::*;
use crate::ID;


#[derive(PartialEq, Clone, AnchorDeserialize, AnchorSerialize)]
pub enum EventMarketType {
    RawDataEventMarket,
    PhaseEventMarket,
    NonPredictEventMarket,
}


#[account]
pub struct EventMarket {
    pub creator: Pubkey,
    pub resolver: Pubkey,
    pub event_type: EventMarketType,
    pub oracle_config: Pubkey,
    pub mint: Option<Pubkey>,
    pub open_raw_data: Option<u64>,
    pub open_phase: Option<u8>,
    pub option: u8,
    pub result: u8,
    pub open_slot: u64,
    pub close_ts: u64,
    pub expiry_ts: u64,
    pub is_opened: bool,
    pub metadata_json_url: String,
    pub bump: u8,
}

impl EventMarket {

    pub const LEN: usize = 32 + 32 + 1 + 32 + 33 + 9 + 2 + 1 + 1 + 8 * 3 + 1 + 150 + 1;
    pub const EVENT_MARKET_SEED: &'static [u8] = b"event_market";

    pub fn find_event_market_account(oracle_config_pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::EVENT_MARKET_SEED,
                oracle_config_pubkey.as_ref()],
            &ID
        )
    }
}