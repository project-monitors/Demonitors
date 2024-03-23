use crate::state::{EventConfig, EventMarket};
use crate::utils::get_oracle_data_account_pubkey;
use crate::error::ErrorCode;
use crate::chain_event::*;

use anchor_lang::prelude::*;
use monitor::state::{OracleConfig, OracleData};


//TODO: Resolver should be a independent program
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct FearAndGreedEventMarketResolveParams {
    pub prize: u64
}

#[derive(Accounts)]
#[instruction(params: FearAndGreedEventMarketResolveParams)]
pub struct FearAndGreedEventMarketResolve<'info> {
    #[account(
    address = event_config.resolver @ ErrorCode::Unauthorized )]
    pub resolver: Signer<'info>,
    #[account(
    address = oracle_data.config @ ErrorCode::UnexpectedAccount)]
    pub oracle_config: Account<'info, OracleConfig>,
    #[account(
    address = get_oracle_data_account_pubkey(oracle_config.key()) @ ErrorCode::UnexpectedAccount )]
    pub oracle_data: Account<'info, OracleData>,
    #[account(
    address = event_market.event_config @ ErrorCode::UnexpectedAccount)]
    pub event_config: Account<'info, EventConfig>,
    #[account(
    mut)]
    pub event_market: Account<'info, EventMarket>,
}

impl<'info> FearAndGreedEventMarketResolve<'info> {

    pub fn process(
        &mut self,
        params: FearAndGreedEventMarketResolveParams
    ) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        require!(now > 0, ErrorCode::Overflow);
        let now_u64 = now as u64;
        require!(now_u64 > self.event_market.close_ts, ErrorCode::EventIsOngoing);
        require!(now_u64 > self.event_market.expiry_ts, ErrorCode::EventIsOngoing);
        require_eq!(self.event_market.result, 0, ErrorCode::EventIsFinalized);
        let raw_data = self.oracle_data.raw_data;
        let raw_data_old = self.event_market.open_raw_data.ok_or(
            ErrorCode::EventMarketDataError)?;
        let event_market = &mut self.event_market;
        if raw_data_old > raw_data {
            event_market.result = 1;
        } else {
            event_market.result = 2;
        }
        event_market.prize = params.prize;
        event_market.is_opened = false;
        emit!(
            EventEvent{
                event_type: EventEventType::Finalize,
                event_config: self.event_config.key(),
                event: Some(event_market.key()),
            }
        );

        Ok(())
    }
}