use std::{sync::Arc};

use factory::{
    accounts as event_accounts,
    instruction as event_instructions,
    state::*,
    instructions::*,
    instructions::resolvers::FearAndGreedEventMarketResolveParams
};

use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
        pubkey,
        signature::{
            Keypair,
            Signature,
        },
        system_program::ID as SYSTEM_PROGRAM_ID,
        sysvar::instructions::ID as SYSVAR_ID,
        sysvar::rent::ID as RENT_ID,
    },
    Program,
};

use anchor_spl::{
    token_interface::ID as TOKEN_2022_PROGRAM_ID,
    associated_token::ID as ASSOCIATED_TOKEN_PROGRAM_ID,
    metadata::ID as MPL_TOKEN_METADATA_ID,
    associated_token::get_associated_token_address_with_program_id
};
use anyhow::Result;

use crate::chain::caller::{ChainClient, setup_client};
use crate::core::{conf::ClientConfig};


pub struct EventCaller {
    pub client: ChainClient,
    pub config: Arc<ClientConfig>,
    pub program: Program<Arc<Keypair>>,
    pub payer: Pubkey
}

impl EventCaller {

    const GLOBAL_CONFIG_SEED: &'static [u8] = b"global_config";
    pub const VISION_MINING_SEED: &'static [u8] = b"ft_vision_mining_pda";
    pub const EVENT_MINING_SEED: &'static [u8] = b"ft_event_mining_pda";
    pub const STAKE_MINING_SEED: &'static [u8] = b"ft_stake_mining_pda";
    pub const MINT_SEED: &'static [u8] = b"mint";
    pub const SBT_MINT_SEED: &'static [u8] = b"sbt_mint";
    pub const MINT_CONFIG_SEED: &'static [u8] = b"mint_config";
    pub const AUTHORITY_SEED: &'static [u8] = b"authority";
    pub const SBT_COLLECTION_SEED: &'static [u8] = b"collection";
    pub const ORACLE_CONFIG_SEED: &'static [u8] = b"oracle-config";
    pub const ORACLE_DATA_SEED: &'static [u8] = b"oracle-data";
    pub const EVENT_SEED: &'static [u8] = b"event";
    pub const EVENT_MARKET_SEED: &'static [u8] = b"event_market";
    pub const USER_POSITION_SEEDS: &'static [u8] = b"user_position";
    pub const EVENT_POSITION_SEEDS: &'static [u8] = b"event_position";
    pub const MARKER_SEED: &'static [u8] = b"marker";


    pub fn new (cfg: Arc<ClientConfig>) -> Result<Self> {
        let config_clone = cfg.clone();
        let client = setup_client(&config_clone)?;
        let program = client.program(pubkey!(cfg.event.program_id).parse()?)?;
        let payer = program.payer();

        Ok(Self {
            client,
            config: cfg,
            program,
            payer,
        })
    }

    pub fn get_pda(program_pubkey: Pubkey, seeds: &[&[u8]]) -> Result<Pubkey> {
        let (pubkey, _) = Pubkey::find_program_address(
            seeds,
            &program_pubkey,
        );
        Ok(pubkey)
    }

    pub fn get_const_name_pda(&self, name: &[u8]) -> Result<Pubkey> {
        Self::get_pda(pubkey!(self.config.event.program_id).parse()?, &[name])
    }

    pub fn get_authority(&self, mint_pubkey: &Pubkey) -> Result<Pubkey> {
        let seeds: &[&[u8]] = &[
            Self::AUTHORITY_SEED,
            mint_pubkey.as_ref(),
        ];
        Self::get_pda(pubkey!(self.config.event.program_id).parse()?, seeds)
    }

    pub fn get_metadata(&self, mint_pubkey: &Pubkey) -> Result<Pubkey> {
        let seeds: &[&[u8]] = &[
            "metadata".as_bytes(),
            MPL_TOKEN_METADATA_ID.as_ref(),
            mint_pubkey.as_ref(),
        ];
        Self::get_pda(MPL_TOKEN_METADATA_ID, seeds)
    }

    pub fn get_edition(&self, mint_pubkey: &Pubkey) -> Result<Pubkey> {
        let seeds: &[&[u8]] = &[
            "metadata".as_bytes(),
            MPL_TOKEN_METADATA_ID.as_ref(),
            mint_pubkey.as_ref(),
            "edition".as_bytes(),
        ];
        Self::get_pda(MPL_TOKEN_METADATA_ID, seeds)
    }

    pub fn get_token_account(&self, user_pubkey: &Pubkey, mint_pubkey: &Pubkey) -> Result<Pubkey> {
        Ok(
            get_associated_token_address_with_program_id(
                user_pubkey, mint_pubkey, &TOKEN_2022_PROGRAM_ID))
    }

    pub fn get_oracle_config(&self, name: &str) -> Result<Pubkey> {
        let seeds: &[&[u8]] = &[
            Self::ORACLE_CONFIG_SEED,
            name.as_bytes(),
        ];
        Self::get_pda(pubkey!(self.config.oracle.program_id).parse()?, seeds)
    }

    pub fn get_oracle_data(&self, name: &str) -> Result<Pubkey> {
        let oracle_config = self.get_oracle_config(name)?;
        let seeds: &[&[u8]] = &[
            Self::ORACLE_DATA_SEED,
            oracle_config.as_ref(),
        ];
        Self::get_pda(pubkey!(self.config.oracle.program_id).parse()?, seeds)
    }

    pub fn get_event_config(&self, name: &str) -> Result<Pubkey> {
        let oracle_config = self.get_oracle_config(name)?;
        let seeds: &[&[u8]] = &[
            Self::EVENT_SEED,
            oracle_config.as_ref(),
        ];
        Self::get_pda(pubkey!(self.config.event.program_id).parse()?, seeds)
    }

    pub fn get_event_market(&self, name: &str, open_ts: u64) -> Result<Pubkey> {
        let oracle_config = self.get_oracle_config(name)?;
        let seeds: &[&[u8]] = &[
            Self::EVENT_MARKET_SEED,
            oracle_config.as_ref(),
            &open_ts.to_be_bytes()[..]
        ];
        Self::get_pda(pubkey!(self.config.event.program_id).parse()?, seeds)
    }

    pub fn get_event_sbt_master_edition_mint(&self, event_config: &Pubkey, option: u8)-> Result<Pubkey> {
        let seeds: &[&[u8]] = &[
            Self::SBT_MINT_SEED,
            event_config.as_ref(),
            &option.to_be_bytes()[..]
        ];
        Self::get_pda(pubkey!(self.config.event.program_id).parse()?, seeds)
    }

    pub fn initialize_global_config(&self) -> Result<Signature> {
        let ix = self.program
            .request()
            .accounts(event_accounts::InitializeGlobalConfig {
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                user: self.payer,
                vision_mining_pda: self.get_const_name_pda(Self::VISION_MINING_SEED)?,
                event_mining_pda: self.get_const_name_pda(Self::EVENT_MINING_SEED)?,
                stake_mining_pda: self.get_const_name_pda(Self::STAKE_MINING_SEED)?,
                vision_mining_admin_pubkey: pubkey!(self.config.event.vision_mining_admin_pubkey).parse()?,
                governor: self.payer,
                system_program: SYSTEM_PROGRAM_ID
            })
            .args(event_instructions::InitializeGlobalConfig{});
        let sig = ix.send()?;
        Ok(sig)
    }
    
    pub fn initialize_mint(&self) -> Result<Signature> {
        let mint_pubkey = self.get_const_name_pda(Self::MINT_SEED)?;
        let ix = self.program
            .request()
            .accounts(event_accounts::InitializeMint{
                payer: self.payer,
                authority: self.get_authority(&mint_pubkey)?,
                mint: mint_pubkey,
                mint_config: self.get_const_name_pda(Self::MINT_CONFIG_SEED)?,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                metadata_account: self.get_metadata(&mint_pubkey)?,
                token_program: TOKEN_2022_PROGRAM_ID,
                token_metadata_program: MPL_TOKEN_METADATA_ID,
                associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
                sysvar_instruction: SYSVAR_ID,
                rent: RENT_ID,
            })
            .args(event_instructions::InitializeMint{ params: InitializeMintParams {
                name: self.config.metadata.ft_token_name.to_string(),
                symbol: self.config.metadata.ft_token_symbol.to_string(),
                uri: self.config.metadata.ft_token_url.to_string(),
                decimals: self.config.metadata.ft_token_decimal,
            }});
        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn mint_tokens(&self, destination_pubkey: Pubkey, amount: u64) -> Result<Signature> {
        let mint_pubkey = self.get_const_name_pda(Self::MINT_SEED)?;
        let ix = self.program
            .request()
            .accounts(event_accounts::MintTokens{
                payer: self.payer,
                global_config:  self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                mint: mint_pubkey,
                user: destination_pubkey,
                token_account: self.get_token_account(&destination_pubkey, &mint_pubkey)?,
                authority: self.get_authority(&mint_pubkey)?,
                token_program: TOKEN_2022_PROGRAM_ID,
                associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            }).args(event_instructions::MintTokens{ params: MintTokensParams { amount } });
        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn initialize_collection(&self) -> Result<Signature> {
        let mint_pubkey = self.get_const_name_pda(Self::SBT_COLLECTION_SEED)?;
        let ix = self.program
            .request()
            .accounts(event_accounts::InitializeCollection{
                payer: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                authority: self.get_authority(&mint_pubkey)?,
                mint: mint_pubkey,
                metadata: self.get_metadata(&mint_pubkey)?,
                master_edition: self.get_edition(&mint_pubkey)?,
                system_program: SYSTEM_PROGRAM_ID,
                token_program: TOKEN_2022_PROGRAM_ID,
                token_metadata_program: MPL_TOKEN_METADATA_ID,
                sysvar_instruction: SYSVAR_ID,
                rent: RENT_ID,
            })
            .args(event_instructions::InitializeCollection{ params: InitializeCollectionParams {
                name: self.config.metadata.sbt_token_name.to_string(),
                symbol: self.config.metadata.sbt_token_symbol.to_string(),
                uri: self.config.metadata.sbt_token_url.to_string(),
            }});
        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn create_event_config(&self, option: u8, meta_json_url: String) -> Result<Signature> {
        let ix = self.program
            .request()
            .accounts(event_accounts::CreateEventConfig{
                payer: self.payer,
                governor: self.payer,
                resolver: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                oracle_config: self.get_oracle_config(&self.config.oracle.config_name)?,
                event_config: self.get_event_config(&self.config.oracle.config_name)?,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(event_instructions::CreateEventConfig{ params: CreateEventConfigParams {
                event_type: EventType::RawDataEventMarket,
                option,
                metadata_json_url: meta_json_url,
            } });
        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn create_event_market(
            &self,
            open_ts: u64,
            close_ts: u64,
            expiry_ts: u64) -> Result<Signature> {
        let ix = self.program
            .request()
            .accounts(event_accounts::CreateEventMarket {
                payer: self.payer,
                governor: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                event_config: self.get_event_config(&self.config.oracle.config_name)?,
                oracle_config: self.get_oracle_config(&self.config.oracle.config_name)?,
                oracle_data: self.get_oracle_data(&self.config.oracle.config_name)?,
                event_market_account: self.get_event_market(&self.config.oracle.config_name, open_ts)?,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(event_instructions::CreateEventMarket {
                params: CreateEventMarketParams {
                    open_ts,
                    close_ts,
                    expiry_ts
                }
            });
        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn toggle_event_market(
            &self, open_ts: u64,
            toggle: bool,
            fetch_oracle_data: bool) -> Result<Signature> {
        let ix = self.program
            .request()
            .accounts(event_accounts::ToggleEventMarket{
                payer: self.payer,
                governor: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                event_config: self.get_event_config(&self.config.oracle.config_name)?,
                oracle_data: self.get_oracle_data(&self.config.oracle.config_name)?,
                event_market_account: self.get_event_market(&self.config.oracle.config_name, open_ts)?,
            })
            .args(event_instructions::ToggleEventMarket{ params: ToggleEventMarketParams {
                toggle, fetch_oracle_data }});

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn create_event_sbt(&self, params: CreateEventSBTParams) -> Result<Signature> {

        let collection_mint_key = self.get_const_name_pda(Self::SBT_COLLECTION_SEED)?;
        let event_config = self.get_event_config(&self.config.oracle.config_name)?;
        let event_sbt_master_edition_mint = self.get_event_sbt_master_edition_mint(&event_config, params.option)?;

        let ix = self.program
            .request()
            .accounts(event_accounts::CreateEventSBT {
                payer: self.payer,
                governor: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                event_config,
                authority: self.get_authority(&collection_mint_key)?,
                collection_mint: collection_mint_key,
                mint: event_sbt_master_edition_mint,
                metadata: self.get_metadata(&event_sbt_master_edition_mint)?,
                master_edition: self.get_edition(&event_sbt_master_edition_mint)?,
                system_program: SYSTEM_PROGRAM_ID,
                token_program: TOKEN_2022_PROGRAM_ID,
                token_metadata_program: MPL_TOKEN_METADATA_ID,
                sysvar_instruction: SYSVAR_ID,
                rent: RENT_ID,
            })
            .args(event_instructions::CreateEventSbt { params });

        let sig = ix.send()?;

        Ok(sig)
    }

    pub fn mint_event_sbt_master_edition(&self, option: u8) -> Result<Signature> {
        let collection_mint_key = self.get_const_name_pda(Self::SBT_COLLECTION_SEED)?;
        let event_config = self.get_event_config(&self.config.oracle.config_name)?;
        let event_sbt_master_edition_mint = self.get_event_sbt_master_edition_mint(&event_config, option)?;

        let ix = self.program
            .request()
            .accounts(event_accounts::MintEventSBTMasterEdition {
                payer: self.payer,
                governor: self.payer,
                global_config: self.get_const_name_pda(Self::GLOBAL_CONFIG_SEED)?,
                event_config,
                authority: self.get_authority(&collection_mint_key)?,
                collection_mint: collection_mint_key,
                mint: event_sbt_master_edition_mint,
                token_account: self.get_token_account(&self.get_authority(&collection_mint_key)?, &event_sbt_master_edition_mint)?,
                metadata: self.get_metadata(&event_sbt_master_edition_mint)?,
                master_edition: self.get_edition(&event_sbt_master_edition_mint)?,
                system_program: SYSTEM_PROGRAM_ID,
                token_program: TOKEN_2022_PROGRAM_ID,
                token_metadata_program: MPL_TOKEN_METADATA_ID,
                sysvar_instruction: SYSVAR_ID,
                associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            })
            .args(event_instructions::MintEventSbtMasterEdition {
                params: MintEventSBTMasterEditionParams { option }
            });

        let sig = ix.send()?;

        Ok(sig)
    }

    pub fn resolve(&self, open_ts: u64, prize: u64) -> Result<Signature> {
        let ix = self.program
            .request()
            .accounts(event_accounts::FearAndGreedEventMarketResolve {
                resolver: self.payer,
                oracle_config: self.get_oracle_config(&self.config.oracle.config_name)?,
                oracle_data: self.get_oracle_data(&self.config.oracle.config_name)?,
                event_config: self.get_event_config(&self.config.oracle.config_name)?,
                event_market: self.get_event_market(&self.config.oracle.config_name, open_ts)?
            })
            .args(event_instructions::Resolve {
                params: FearAndGreedEventMarketResolveParams {
                    prize
                }
            });

        let sig = ix.send()?;

        Ok(sig)
    }
}