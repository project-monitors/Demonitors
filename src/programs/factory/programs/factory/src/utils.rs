use std::ops::DerefMut;
use anchor_lang::prelude::*;
use monitor::ID as MONITOR_PROGRAM_ID;
use crate::error::ErrorCode;

pub fn close_account(account: &AccountInfo, dest_account: &AccountInfo) -> Result<()> {
    //close account by draining lamports
    let dest_starting_lamports = dest_account.lamports();
    **dest_account.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(account.lamports())
        .ok_or(ErrorCode::CloseAccountFailed)?;
    **account.lamports.borrow_mut() = 0;
    let mut data = account.try_borrow_mut_data()?;
    for byte in data.deref_mut().iter_mut() {
        *byte = 0;
    }
    Ok(())
}

pub(crate) fn get_oracle_data_account_pubkey(oracle_config_pubkey: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[ b"oracle-data", oracle_config_pubkey.as_ref()],
        &MONITOR_PROGRAM_ID
    ).0
}