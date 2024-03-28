use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientConfig {
    pub solana: SolanaConfig,
    pub oracle: OracleConfig,
    pub event: EventConfig,
    pub market: Market,
    pub metadata: Metadata,
    pub tokenomics: Tokenomics,
    pub api: Api
}

#[derive(Deserialize, Clone, Debug)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub keypair_path: String,
    pub commitment: String,
    pub network: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OracleConfig {
    pub config_name: String,
    pub config_description: String,
    pub total_phase: u8,
    pub authority_pubkey: String,
    pub interval: u64,
    pub program_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EventConfig {
    pub program_id: String,
    pub vision_mining_admin_pubkey: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Market {
    pub hourly_create: bool,
    pub daily_create: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Metadata {
    pub ft_token_name: String,
    pub ft_token_symbol: String,
    pub ft_token_url: String,
    pub ft_token_decimal: u8,
    pub sbt_token_name: String,
    pub sbt_token_symbol: String,
    pub sbt_token_url: String,
    pub event_metadata_url: String,
    pub fear_sbt_token_name: String,
    pub fear_sbt_token_symbol: String,
    pub fear_sbt_token_url: String,
    pub greed_sbt_token_name: String,
    pub greed_sbt_token_symbol: String,
    pub greed_sbt_token_url: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tokenomics {
    pub vision_mining: u64,
    pub event_mining: u64,
    pub event_prize: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Api {
    pub port: u64,
}

