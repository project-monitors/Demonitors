use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientConfig {
    pub solana: SolanaConfig,
    pub oracle: OracleConfig,
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
}
