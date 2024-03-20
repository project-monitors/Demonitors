use anchor_lang::prelude::*;


#[event]
pub struct EventMarketEvent {
    pub event_type: EventMarketEventType,
    pub market: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum EventMarketEventType {
    Create,
    Open,
    Finalize
}