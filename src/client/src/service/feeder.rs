use std::sync::Arc;
use crate::prelude::*;
use crate::service::fetcher::FetcherFNG;
use anyhow::Result;
use tokio::task::spawn_blocking;

pub struct Feeder {
    pub caller: ChainCaller,
    pub fetcher: FetcherFNG,
    pub latest_timestamp: u64,
}

impl Feeder{

    pub fn new(cfg: Arc<ClientConfig>) -> Result<Feeder> {
        let caller = ChainCaller::new(cfg)?;
        let fetcher = FetcherFNG::new();
        let latest_timestamp: u64 = 0;
        Ok(Feeder{
            caller,
            fetcher,
            latest_timestamp
        })
    }

    pub async fn fake_feed(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn feed(&mut self) -> Result<()> {
        println!("[Debug] feed begin...");
        self.fetcher.fetch_fear_and_greed_index().await?;
        println!("[Debug] data fetched...");
        let fetched_data = &self.fetcher.index.data[0];
        let ts = fetched_data.timestamp.clone().parse::<u64>()?;
        if ts > self.latest_timestamp {
            let data = self.caller.get_oracle_data()?;
            println!("[Debug] get oracle data ...");
            if ts > data.timestamp {
                // compute delta
                let latest_raw_data = fetched_data.value.parse::<u64>()?;
                let raw_data = latest_raw_data;
                let decimals = 0;
                let phase: u8;
                if latest_raw_data - data.raw_data >= 0 {
                    phase = 1
                } else {
                    phase = 0
                }
                let request = OracleDataRequest {
                    phase,
                    raw_data,
                    decimals
                };
                let sig = self.caller.set_oracle_data(&request)?;
                println!("[INFO] Update solana oracle data account successfully {}", sig);
            }
            self.latest_timestamp = ts;
        }
        Ok(())

    }

}