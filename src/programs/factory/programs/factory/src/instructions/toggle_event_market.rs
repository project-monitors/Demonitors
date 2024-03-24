use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use monitor::state::OracleData;
use crate::chain_event::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct ToggleEventMarketParams{
    pub toggle: bool,
    pub fetch_oracle_data: bool
}

#[derive(Accounts)]
#[instruction(params: ToggleEventMarketParams)]
pub struct ToggleEventMarket<'info> {
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
    pub oracle_data: Account<'info, OracleData>,
    #[account(
    address = event_market_account.event_config @ ErrorCode::UnexpectedAccount)]
    pub event_config: Account<'info, EventConfig>,
    #[account(
    mut)]
    pub event_market_account: Account<'info, EventMarket>
}

impl<'info> ToggleEventMarket<'info> {

    pub fn process(
        &mut self,
        params: ToggleEventMarketParams
    ) -> Result<()> {
        let event_market = &mut self.event_market_account;
        require_eq!(event_market.result, 0, ErrorCode::EventIsFinalized);
        if params.fetch_oracle_data {
            let raw_data = self.oracle_data.raw_data;
            let phase = self.oracle_data.phase;
            event_market.open_raw_data = Some(raw_data);
            event_market.open_phase = Some(phase);
        }
        if params.toggle {
            event_market.is_opened = !event_market.is_opened;
            if event_market.is_opened == true {
                emit!(EventEvent{
                event_type: EventEventType::Open,
                event_config: self.event_config.key(),
                event: Some(self.event_market_account.key())
            })
            }
        }
        Ok(())
    }
}