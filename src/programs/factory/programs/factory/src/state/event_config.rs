use anchor_lang::prelude::*;
use crate::state::*;
use crate::ID;


#[derive(PartialEq, Clone, AnchorDeserialize, AnchorSerialize)]
pub enum EventType {
    RawDataEventMarket,
    PhaseEventMarket,
    NonPredictEvent
}


#[account]
pub struct EventConfig {
    pub creator: Pubkey,
    pub resolver: Pubkey,
    pub event_type: EventType,
    pub oracle_config: Pubkey,
    pub option: u8,
    pub index: u64,
    pub bump: u8,
    pub metadata_json_url: UriResource,
}

impl EventConfig {

    pub const LEN: usize = 32 + 32 + 1 + 32 + 1 + 8 + 1 + UriResource::LEN;
    pub const EVENT_SEED: &'static [u8] = b"event";

    pub fn find_event_config_account(oracle_config_pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::EVENT_SEED,
                oracle_config_pubkey.as_ref()],
            &ID
        )
    }
}