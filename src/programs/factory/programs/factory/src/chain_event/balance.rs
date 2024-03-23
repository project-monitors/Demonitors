use anchor_lang::prelude::*;

// Events
#[event]
pub struct BalanceChangeEvent {
    pub event_type: BalanceChangeEventType,
    pub mint: Pubkey,
    pub from_token_account: Option<Pubkey>,
    pub from_change: Option<U64ValueChange>,
    pub to_token_account: Option<Pubkey>,
    pub to_change: Option<U64ValueChange>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum BalanceChangeEventType {
    Mint,
    Transfer,
    Burn
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct U64ValueChange {
    pub old: u64,
    pub new: u64,
}
