use crate::state::*;
use crate::error::ErrorCode;
use crate::utils::get_oracle_data_account_pubkey;
use crate::event::*;
use monitor::{
    state::{OracleConfig, OracleData},
    ID as MONITOR_PROGRAM_ID,
};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::id as INSTRUCTIONS_ID,
};
use anchor_spl::{
    token_interface::{Mint, TokenInterface },
    metadata::{
        Metadata,
    },
    metadata::mpl_token_metadata::{
        types::{
            TokenStandard, CollectionDetails, Creator, PrintSupply
        },
        instructions::{
            CreateV1CpiBuilder
        },
    },
};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateEventMarketParams {
    pub event_type: EventMarketType,
    pub option: u8,
    pub close_ts: u64,
    pub expiry_ts: u64,
    pub metadata_json_url: String
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
    /// CHECK: This is safe and we will not read or write this account
    pub resolver: UncheckedAccount<'info>,
    #[account(
    seeds = [GlobalConfig::GLOBAL_CONFIG_SEED],
    bump = global_config.global_config_bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
    address = oracle_data.config @ ErrorCode::ConfigMismatched)]
    pub oracle_config: Account<'info, OracleConfig>,
    pub oracle_data: Account<'info, OracleData>,
    pub mint: Option<InterfaceAccount<'info, Mint>>,
    #[account(
    init,
    payer = payer,
    space = 8 + EventMarket::LEN,
    seeds = [ EventMarket::EVENT_MARKET_SEED, &oracle_config.key().to_bytes()],
    bump)]
    pub event_market_account: Account<'info, EventMarket>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateEventMarket<'info> {

    pub fn process(
        &mut self,
        params: CreateEventMarketParams
    ) -> Result<()> {
        let oracle_data_pubkey = get_oracle_data_account_pubkey(self.oracle_config.key());
        require_keys_eq!(oracle_data_pubkey, self.oracle_data.key(), ErrorCode::UnexpectedAccount);
        //TODO: support FT deposit
        require!(self.mint.is_none(), ErrorCode::UnsupportedNow);
        //TODO: support new event type
        require!(params.event_type == EventMarketType::RawDataEventMarket, ErrorCode::UnsupportedNow);
        require!(params.metadata_json_url.as_bytes().len() < 150, ErrorCode::StringTooLong);
        let clock = Clock::get()?;
        let open_slot = clock.slot;
        let now = clock.unix_timestamp;
        require!(now > 0, ErrorCode::Overflow);
        let now_u64 = now as u64;
        require!(params.close_ts > now_u64, ErrorCode::InvalidArgument);
        require!(params.expiry_ts > now_u64, ErrorCode::InvalidArgument);
        let oracle_data = &self.oracle_data;
        let oracle_config = &self.oracle_config;
        require!(oracle_data.timestamp > 0, ErrorCode::OracleDataError);

        let (event_market_pubkey, bump) =
            EventMarket::find_event_market_account(oracle_config.key());

        let market = &mut self.event_market_account;
        market.creator = self.payer.key();
        market.resolver = self.resolver.key();
        market.event_type = params.event_type;
        market.oracle_config = self.oracle_config.key();
        //TODO: support FT deposit
        market.mint = None;
        market.open_raw_data = Some(oracle_data.raw_data);
        market.open_phase = Some(oracle_data.phase);
        market.option = params.option;
        market.result = 0;
        market.open_slot = open_slot;
        market.close_ts = params.close_ts;
        market.expiry_ts = params.expiry_ts;
        market.is_opened = false;
        market.metadata_json_url = params.metadata_json_url;
        market.bump = bump;

        emit!(EventMarketEvent{
            event_type: EventMarketEventType::Create,
            market: event_market_pubkey
        });

        Ok(())
    }
}