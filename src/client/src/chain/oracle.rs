use std::str::FromStr;
use std::{sync::Arc};

use monitor::{
    accounts as monitor_accounts,
    instruction as monitor_instructions,
    ID as monitor_program_id,
    state::{
        OracleConfig,
        OracleData
    }
};

use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
        signature::{
            Keypair,
            Signature,
            },
        system_program::ID as system_program_id,
    },
    Program,
    anchor_lang::AccountDeserialize
};

use anyhow::{Result, anyhow, Error};

use crate::chain::caller::{ChainClient, setup_client};
use crate::chain::{ORACLE_CONFIG_SEED_STRING, ORACLE_DATA_SEED_STRING};
use crate::core::{conf::ClientConfig, error::ClientError};


pub struct OracleDataRequest {
    pub phase: u8,
    pub raw_data: u64,
    pub decimals: u8
}

pub struct ChainCaller {
    pub client: ChainClient,
    pub config: Arc<ClientConfig>,
    pub program: Program<Arc<Keypair>>,
    pub payer: Pubkey,
    pub config_pda: (Pubkey, u8),
    pub data_pda: (Pubkey, u8)
}

impl ChainCaller{

    pub fn new (cfg: Arc<ClientConfig>) -> Result<ChainCaller> {
        let config_clone = cfg.clone();
        let client = setup_client(&config_clone)?;
        let program = client.program(monitor_program_id)?;
        let payer = program.payer();
        let config_pda = Pubkey::find_program_address(
            &[
                ORACLE_CONFIG_SEED_STRING.as_bytes(),
                cfg.oracle.config_name.as_bytes(),
            ],
            &monitor_program_id,
        );
        let data_pda = Pubkey::find_program_address(
            &[
                ORACLE_DATA_SEED_STRING.as_bytes(),
                config_pda.0.as_ref(),
            ],
            &monitor_program_id,
        );

        Ok(ChainCaller {
            client,
            config: cfg,
            program,
            payer,
            config_pda,
            data_pda
        })
    }

    pub fn initialize_oracle_config(&self) -> Result<Signature> {

        if self.config.oracle.config_name.len() > 31 {
            return Err(ClientError::InvalidParam("name".to_string()).into());
        }
        if self.config.oracle.config_description.len() > 200 {
            return Err(ClientError::InvalidParam("description".to_string()).into());
        }

        let ix = self.program
            .request()
            .accounts(monitor_accounts::InitializeOracleConfig {
                config: self.config_pda.0,
                user: self.payer,
                authority_pubkey: Pubkey::from_str(self.config.oracle.authority_pubkey.as_str())?,
                system_program: system_program_id,
            })
            .args(monitor_instructions::InitializeOracleConfig {
                name: self.config.oracle.config_name.clone(),
                description: self.config.oracle.config_description.clone(),
                total_phase: self.config.oracle.total_phase.clone(),
            });

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn add_authority_to_oracle_config(&self, should_be_added: Pubkey) -> Result<Signature> {
        let ix = self.program.request()
            .accounts(monitor_accounts::AddAuthorityToOracleConfig{
                config: self.config_pda.0,
                user: self.payer,
                authority_pubkey: should_be_added,
            })
            .args(monitor_instructions::AddAuthorityToOracleConfig{});

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn remove_authority_from_oracle_config(&self, should_be_removed: Pubkey) -> Result<Signature> {
        let ix = self.program.request()
            .accounts(monitor_accounts::RemoveAuthorityFromOracleConfig{
                config: self.config_pda.0,
                user: self.payer,
                authority_pubkey: should_be_removed,
            })
            .args(monitor_instructions::RemoveAuthorityFromOracleConfig{});

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn get_oracle_config(&self) -> Result<OracleConfig> {
        let oracle_config_account_data = self.program.rpc()
            .get_account_data(&self.config_pda.0)
            .map_err(|_| {
                anyhow!(
                    "Couldn't find oracle config account: {}",
                    &self.config_pda.0.to_string()
                )
            })?;
        let oracle_config =  OracleConfig::try_deserialize(&mut oracle_config_account_data.as_slice());

        match oracle_config {
            Ok(o)=> {
                Ok(o)
            }
            Err(e) => {
                Err(Error::new(e))
            }
        }
    }

    pub fn initialize_oracle_data(&self) -> Result<Signature> {
        let ix = self.program.request()
            .accounts(monitor_accounts::InitializeOracleData{
                config: self.config_pda.0,
                oracle: self.data_pda.0,
                user: self.payer,
                system_program: system_program_id,
            })
            .args(monitor_instructions::InitializeOracleData{});

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn set_oracle_data(&self, request: &OracleDataRequest) -> Result<Signature> {
        let ix = self.program.request()
            .accounts(monitor_accounts::SetOracleData{
                config: self.config_pda.0,
                oracle: self.data_pda.0,
                user: self.payer
            })
            .args(monitor_instructions::SetOracleData{
                phase: request.phase,
                raw_data: request.raw_data,
                decimals: request.decimals,
                bump: self.data_pda.1,
            });

        let sig = ix.send()?;
        Ok(sig)
    }

    pub fn get_oracle_data(&self) -> Result<OracleData> {
        let oracle_data_account_data = self.program.rpc()
            .get_account_data(&self.data_pda.0)
            .map_err(|_| {
                anyhow!(
                    "Couldn't find oracle data account: {}",
                    &self.data_pda.0.to_string()
                )
            })?;
        let oracle_data =  OracleData::try_deserialize(&mut oracle_data_account_data.as_slice());

        match oracle_data {
            Ok(o)=> {
                Ok(o)
            }
            Err(e) => {
                Err(Error::new(e))
            }
        }
    }
}



