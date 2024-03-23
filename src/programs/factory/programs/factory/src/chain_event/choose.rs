use anchor_lang::prelude::*;


#[event]
pub struct ChooseEvent {
    pub event_type: ChooseEventType,
    pub event_market: Pubkey,
    pub user_key: Pubkey,
    pub indicate: u8
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ChooseEventType {
    Choose,
    Withdraw,
    ClaimWithNonPrize,
    ClaimWithPrize,
    ClaimWithPrizeAndSBT
}