use anchor_lang::prelude::*;
use crate::ID;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum PositionType {
    UserSBTPosition,
    EventSBTPosition
}

#[account]
pub struct UserPosition {
    pub position_type: PositionType,
    pub existed: bool,
    pub marker: Pubkey,
}

impl UserPosition {

    pub const LEN: usize = 1 + 1 + 32;
    pub const POSITION_SEEDS: &'static [u8] = b"user_position";

    pub fn find_user_position_account(
        event_market_pubkey: Pubkey,
        user_pubkey: Pubkey,
        mint_pubkey: Option<Pubkey>) -> (Pubkey, u8) {

        let mut seeds = vec![
            Self::POSITION_SEEDS,
            event_market_pubkey.as_ref(),
            user_pubkey.as_ref(),
        ];

        if let Some(ref mint_pk) = mint_pubkey {
            seeds.push(mint_pk.as_ref());
        }

        Pubkey::find_program_address(&seeds, &ID)
    }
}