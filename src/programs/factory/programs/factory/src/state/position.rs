use anchor_lang::prelude::*;
use crate::ID;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum PositionType {
    EventSBTPosition,
    UserSBTPosition,
    EventTokenPosition,
    UserTokenPosition
}

#[account]
pub struct Position {
    pub event_market: Pubkey,
    pub existed: bool,
    pub position_type: PositionType,
    pub marker: Option<Pubkey>,
    pub amount: Option<u64>,
}

impl Position {

    pub const LEN: usize = 32 + 1 + 1 + 33 + 9;
    pub const POSITION_SEEDS: &'static [u8] = b"user_position";

    pub fn find_user_position_account(
        event_market_pubkey: Pubkey,
        option_index: u8,
        user_pubkey: Option<Pubkey>,
        mint_pubkey: Option<Pubkey>) -> (Pubkey, u8) {
        let option = &[option_index];
        let mut seeds = vec![
            Self::POSITION_SEEDS,
            event_market_pubkey.as_ref(),
            option,
        ];

        if let Some(ref user_pk) = user_pubkey {
            seeds.push(user_pk.as_ref());
        }

        if let Some(ref mint_pk) = mint_pubkey {
            seeds.push(mint_pk.as_ref());
        }

        Pubkey::find_program_address(&seeds, &ID)
    }
}