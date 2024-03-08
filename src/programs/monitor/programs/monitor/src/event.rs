use anchor_lang::prelude::*;

// Events
#[event]
pub struct SetOracleDataEvent {
    pub state: Pubkey,
    pub phase_change: Option<U8ValueChange>,
    pub raw_data_change: Option<U64ValueChange>,
    pub decimals_change: Option<U8ValueChange>,
    pub timestamp_change: Option<U64ValueChange>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct U64ValueChange {
    pub old: u64,
    pub new: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct U8ValueChange {
    pub old: u8,
    pub new: u8,
}