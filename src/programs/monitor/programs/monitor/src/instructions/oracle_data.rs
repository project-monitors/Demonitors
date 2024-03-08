use anchor_lang::prelude::*;
use crate::state::{OracleConfig, OracleData};
use crate::error::ErrorCode;
use crate::event::*;


#[derive(Accounts)]
pub struct InitializeOracleData<'info> {
    pub config: Account<'info, OracleConfig>,
    #[account(
    init,
    payer = user,
    space = 8 + 32 + 1 + 8 + 1 + 8 + 1,
    seeds = [b"oracle-data", config.key().as_ref()],
    bump)]
    pub oracle: Account<'info, OracleData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeOracleData<'info> {

    pub fn process(&mut self, bump: u8) -> Result<()> {
        require!(self.config.has_authority(self.user.key), ErrorCode::Unauthorized);
        let oracle_data = &mut self.oracle;
        oracle_data.config = self.config.key();
        oracle_data.bump = bump;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetOracleData<'info> {
    pub config: Account<'info, OracleConfig>,
    #[account(
    mut,
    seeds = [b"oracle-data", config.key().as_ref()],
    bump = oracle.bump)]
    pub oracle: Account<'info, OracleData>,
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> SetOracleData<'info>  {

    pub fn process(&mut self, phase: u8, raw_data: u64, decimals: u8, bump: u8) -> Result<()> {
        require_eq!(self.oracle.bump, bump, ErrorCode::InvalidArgument);
        require_keys_eq!(self.config.key(), self.oracle.config, ErrorCode::ConfigMismatched);
        require!(self.config.has_authority(self.user.key), ErrorCode::Unauthorized);
        require!(phase < self.config.total_phases, ErrorCode::InvalidArgument);
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        require!(current_time > 0, ErrorCode::Overflow);
        let current_time_u64 = current_time as u64;
        let data_account = &mut self.oracle;

        let phase_change = {
            let old = data_account.phase;
            if old == phase {
                None
            } else {
                data_account.phase = phase;
                Some(U8ValueChange {old, new: phase})
            }
        };

        let raw_data_change = {
            let old = data_account.raw_data;
            if old == raw_data {
                None
            } else {
                data_account.raw_data = raw_data;
                Some(U64ValueChange {old, new: raw_data})
            }
        };

        let decimals_change = {
            let old = data_account.decimals;
            if old == decimals {
                None
            } else {
                data_account.decimals = decimals;
                Some(U8ValueChange {old, new: decimals})
            }
        };

        let timestamp_change = {
            let old = data_account.timestamp;
            if old == current_time_u64 {
                None
            } else {
                data_account.timestamp = current_time_u64;
                Some(U64ValueChange {old, new: current_time_u64})
            }
        };

        emit!(SetOracleDataEvent{
            state: self.oracle.key(),
            phase_change,
            raw_data_change,
            decimals_change,
            timestamp_change,
        });
        Ok(())
    }

}


