use anchor_lang::prelude::*;


#[event]
pub struct SBTMintEvent {
    pub event_type: SBTMintEventType,
    pub mint_key: Pubkey,
    pub user_key: Pubkey,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum SBTMintEventType {
    CreateSBT,
    MintSBT,
    PrintSBT,
}