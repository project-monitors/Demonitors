use anchor_lang::prelude::*;
use crate::ID;

#[account]
pub struct Marker {
    pub indicate: u8,
    pub event_market: Option<Pubkey>,
    pub bump: u8
}

impl Marker {

    pub const LEN: usize = 1 + 33 + 1;
    pub const MARKER_SEED: &'static [u8] = b"marker";

    pub fn find_marker_account(
        user_pubkey: Pubkey, sbt_mint_pubkey: Pubkey) -> (Pubkey, u8) {

        let seeds = vec![
            Self::MARKER_SEED,
            user_pubkey.as_ref(),
            sbt_mint_pubkey.as_ref()];

        Pubkey::find_program_address(&seeds, &ID)
    }

}

