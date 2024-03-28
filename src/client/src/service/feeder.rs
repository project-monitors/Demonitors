use crate::prelude::*;
use crate::service::fetcher::FetcherFNG;
use crate::data::mem_db::*;
use anyhow::Result;


pub struct Feeder {
    pub caller: ChainCaller,
    pub fetcher: FetcherFNG,
    pub latest_timestamp: u64,
    pub db_conn: Conn,
}

impl Feeder{

    pub fn new(cfg: ClientConfig, db_conn: Conn) -> Result<Feeder> {
        let caller = ChainCaller::new(cfg)?;
        let fetcher = FetcherFNG::new();
        let latest_timestamp: u64 = 0;
        Ok(Feeder{
            caller,
            fetcher,
            latest_timestamp,
            db_conn
        })
    }

    pub fn feed(&mut self) -> Result<()> {
        println!("[Debug] feed begin...");
        self.fetcher.fetch_fear_and_greed_index()?;
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
                let phase: u8 = if latest_raw_data >= data.raw_data {
                    1
                } else {
                    0
                };
                let request = OracleDataRequest {
                    phase,
                    raw_data,
                    decimals
                };
                let sig = self.caller.set_oracle_data(&request)?;
                println!("[INFO] Update solana oracle data account successfully {}", sig);

                let new_data = self.caller.get_oracle_data()?;
                self.db_conn.upsert(Some(&new_data), None)?;
            }
            self.latest_timestamp = ts;
        }
        Ok(())

    }

}