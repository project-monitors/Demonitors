use crate::state::*;
use crate::error::ErrorCode;
use crate::chain_event::*;
use monitor::state::OracleConfig;
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateEventConfigParams {
    pub event_type: EventMarketType,
    pub option: u8,
    pub metadata_json_url: String
}

#[derive(Accounts)]
#[instruction(params: CreateEventConfigParams)]
pub struct CreateEventConfig<'info> {
    #[account(
    mut)]
    pub payer: Signer<'info>,
    #[account(
    address = global_config.governor @ ErrorCode::Unauthorized)]
    pub governor: Signer<'info>,
    /// CHECK: This is safe and we will not read or write this account
    pub resolver: UncheckedAccount<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    pub oracle_config: Account<'info, OracleConfig>,
    #[account(
    init,
    payer = payer,
    space = 8 + EventConfig::LEN,
    seeds = [ EventConfig::EVENT_SEED, &oracle_config.key().to_bytes()],
    bump)]
    pub event_config: Account<'info, EventConfig>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateEventConfig<'info> {
    pub fn process(
        &mut self,
        params: CreateEventConfigParams,
        bump: u8
    ) -> Result<()> {
        //TODO: support new chain_event type
        require!(params.event_type == EventMarketType::RawDataEventMarket, ErrorCode::UnsupportedNow);
        let event_config = &mut self.event_config;
        event_config.metadata_json_url = UriResource::validate(&params.metadata_json_url)?;
        event_config.creator = self.payer.key();
        event_config.resolver = self.resolver.key();
        event_config.oracle_config = self.oracle_config.key();
        event_config.option = params.option;
        event_config.index = 0;
        event_config.bump = bump;

        emit!(EventEvent{
            event_type: EventEventType::ConfigCreate,
            event_config: self.event_config.key(),
            event: None
        });

        Ok(())

    }
}