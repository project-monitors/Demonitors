use crate::core::conf::ClientConfig;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::{solana_sdk::signature::{read_keypair_file, Keypair}, Client, Cluster};
use std::sync::Arc;
use anyhow::{
    Result,
    anyhow
};

pub type ChainClient = Client<Arc<Keypair>>;

pub fn setup_client(cfg: &ClientConfig) -> Result<ChainClient> {
    let keypair = read_keypair_file(cfg.solana.keypair_path.clone())
        .map_err(|_| anyhow!("Failed to read keypair file at {}", cfg.solana.keypair_path))?;
    let key_bytes = keypair.to_bytes();
    let signer = Arc::new(Keypair::from_bytes(&key_bytes)
                              .map_err(anyhow::Error::from)?);
    let cluster = Cluster::Custom(cfg.solana.rpc_url.clone(), cfg.solana.ws_url.clone());
    let opts = CommitmentConfig::confirmed();
    Ok(Client::new_with_options(cluster, signer, opts))
}
