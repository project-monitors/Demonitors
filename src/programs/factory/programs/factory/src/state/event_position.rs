use anchor_lang::prelude::*;
use crate::state::user_position::PositionType;
use crate::ID;


#[account]
pub struct EventPosition {
    pub position_type: PositionType,
    pub amount: u64,
}

impl EventPosition {

    pub const LEN: usize = 1 + 8;
    pub const POSITION_SEEDS: &'static [u8] = b"event_position";

    pub fn find_event_position_account(
        event_market_pubkey: Pubkey,
        option_index: u8,
        mint_pubkey: Option<Pubkey>) -> (Pubkey, u8) {

        let binding = [option_index];
        let mut seeds = vec![
            Self::POSITION_SEEDS,
            event_market_pubkey.as_ref(),
            &binding
        ];

        if let Some(ref mint_pk) = mint_pubkey {
            seeds.push(mint_pk.as_ref());
        }

        Pubkey::find_program_address(&seeds, &ID)
    }
}