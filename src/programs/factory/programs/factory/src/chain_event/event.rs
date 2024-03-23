use anchor_lang::prelude::*;


#[event]
pub struct EventEvent {
    pub event_type: EventEventType,
    pub event_config: Pubkey,
    pub event: Option<Pubkey>
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum EventEventType {
    ConfigCreate,
    Create,
    Open,
    Finalize
}