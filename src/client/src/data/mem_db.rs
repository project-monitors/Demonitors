use serde::Serialize;
use {
    byteorder::LittleEndian,
    anyhow::{Result, anyhow},
    sled::*,
    zerocopy::{
        byteorder::U64, AsBytes, FromBytes, Unaligned, LayoutVerified
    },
    monitor::state::OracleData
};



#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct OracleDataValue {
    pub raw_data: U64<LittleEndian>,
    pub timestamp: U64<LittleEndian>,
    pub authoritative: U64<LittleEndian>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OracleDataFetched {
    pub raw_data: u64,
    pub timestamp: u64,
    pub authoritative: bool
}

#[derive(Clone)]
pub struct FearAndGreedApiData {
    pub raw_data: u64,
    pub timestamp: u64
}

#[derive(Clone)]
pub struct Conn {
    pub db: Db,
    pub tree: Tree
}

impl Conn {

    pub fn new() -> Result<Self> {
        let db = open("monitor_db")?;
        Ok(Self {
            db: db.clone(),
            tree: db.open_tree("oracle_data")?
        })
    }

    pub fn upsert(&self, oracle_data: Option<&OracleData>, api_data: Option<FearAndGreedApiData>) -> Result<()> {
        let key: u64;
        let value: OracleDataValue;

        if let Some(oracle_d) = oracle_data {
            key = oracle_d.timestamp;
            value = OracleDataValue {
                raw_data: U64::new(oracle_d.raw_data),
                timestamp: U64::new(oracle_d.timestamp),
                authoritative: U64::new(1),
            };
        } else if let Some(api_d) = api_data{
            key = api_d.timestamp;
            value = OracleDataValue {
                raw_data: U64::new(api_d.raw_data),
                timestamp: U64::new(api_d.timestamp),
                authoritative: U64::new(0),
            };
        } else {
            return Err(anyhow!("not supported params"))
        }

        self.tree.update_and_fetch(key.to_be_bytes(), |value_opt| {
            if let Some(existing) = value_opt {
                let backing_bytes = IVec::from(existing);
                Some(backing_bytes)
            } else {
                Some(IVec::from(value.as_bytes()))
            }
        })?;

        println!("[INFO] save oracle data timestamp {} into memory db", key);
        Ok(())
    }


    pub fn fetch_oracle_data(&self, timestamp: u64) -> Result<OracleDataFetched> {
        let key_before = timestamp.to_be_bytes();
        if let Some((_, value)) = self.tree.get_lt(key_before)? {
            let oracle_data: LayoutVerified<&[u8], OracleDataValue> =
                LayoutVerified::new(&*value).unwrap();
            let raw_data = oracle_data.raw_data.get();
            let timestamp =  oracle_data.timestamp.get();
            let authoritative_u64 = oracle_data.authoritative.get();
            let mut authoritative = false;
            if authoritative_u64 !=0 {
                authoritative = true;
            }
            Ok(OracleDataFetched{
                raw_data,
                timestamp,
                authoritative,
            })
        } else {
            Err(anyhow!("can not find data from memory db"))
        }
    }
}