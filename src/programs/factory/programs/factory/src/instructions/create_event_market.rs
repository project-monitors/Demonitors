use crate::state::*;
use crate::error::ErrorCode;
use crate::utils::get_oracle_data_account_pubkey;
use crate::chain_event::*;
use monitor::{
    state::{OracleConfig, OracleData},
};
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateEventMarketParams {
    pub close_ts: u64,
    pub expiry_ts: u64,
}

#[derive(Accounts)]
#[instruction(params: CreateEventMarketParams)]
pub struct CreateEventMarket<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    #[account(
    address = global_config.governor @ ErrorCode::Unauthorized)]
    pub governor: Signer<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
    mut)]
    pub event_config: Account<'info, EventConfig>,
    #[account(
    address = oracle_data.config @ ErrorCode::ConfigMismatched)]
    pub oracle_config: Account<'info, OracleConfig>,
    pub oracle_data: Account<'info, OracleData>,
    #[account(
    init,
    payer = payer,
    space = 8 + EventMarket::LEN,
    seeds = [ EventMarket::EVENT_MARKET_SEED,
    &oracle_config.key().to_bytes(),
    &event_config.index.to_be_bytes()[..]],
    bump)]
    pub event_market_account: Account<'info, EventMarket>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateEventMarket<'info> {

    pub fn process(
        &mut self,
        params: CreateEventMarketParams,
        bump: u8
    ) -> Result<()> {
        let oracle_data_pubkey = get_oracle_data_account_pubkey(self.oracle_config.key());
        require_keys_eq!(oracle_data_pubkey, self.oracle_data.key(), ErrorCode::UnexpectedAccount);
        let clock = Clock::get()?;
        let open_slot = clock.slot;
        let now = clock.unix_timestamp;
        require!(now > 0, ErrorCode::Overflow);
        let now_u64 = now as u64;
        require!(params.close_ts > now_u64, ErrorCode::InvalidArgument);
        require!(params.expiry_ts > now_u64, ErrorCode::InvalidArgument);
        let oracle_data = &self.oracle_data;
        require!(oracle_data.timestamp > 0, ErrorCode::OracleDataError);

        let market = &mut self.event_market_account;
        market.event_config = self.event_config.key();
        market.open_raw_data = Some(oracle_data.raw_data);
        market.open_phase = Some(oracle_data.phase);
        market.option = self.event_config.option;
        market.result = 0;
        market.prize = 0;
        market.open_slot = open_slot;
        market.close_ts = params.close_ts;
        market.expiry_ts = params.expiry_ts;
        market.is_opened = false;
        market.bump = bump;

        let event_config = &mut self.event_config;
        event_config.index = event_config.index.checked_add(1).ok_or(ErrorCode::Overflow)?;

        emit!(EventEvent{
            event_type: EventEventType::Create,
            event_config: self.event_config.key(),
            event: Some(self.event_market_account.key())
        });

        Ok(())
    }
}