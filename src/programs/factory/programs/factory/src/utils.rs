use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use monitor::ID as MONITOR_PROGRAM_ID;


pub(crate) fn get_oracle_data_account_pubkey(oracle_config_pubkey: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[ b"oracle-data", oracle_config_pubkey.as_ref()],
        &MONITOR_PROGRAM_ID
    ).0
}

//noinspection RsTypeCheck
pub(crate) fn get_ata(user_key: &Pubkey, mint_key: &Pubkey, token_program_id: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(
        user_key,
        mint_key,
        token_program_id)
}
