use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest::blocking::Client;
use crate::core::error::ClientError;

const API_URI: &str = "https://api.alternative.me/fng/";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FearAndGreedIndex {
    pub name: String,
    pub data: Vec<IndexData>,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IndexData {
    pub value: String,
    pub value_classification: String,
    pub timestamp: String,
    pub time_until_update: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Metadata {
    pub error: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default)]
pub struct FetcherFNG {
    pub client: Client,
    pub index: FearAndGreedIndex,
    pub url: String
}

impl FetcherFNG {

    pub fn new() -> FetcherFNG {
        FetcherFNG {
            client: Client::new(),
            url: API_URI.to_string(),
            ..Default::default()
        }
    }

    pub fn fetch_fear_and_greed_index(&mut self) -> Result<()> {
        let url = self.url.clone();
        let res =  self.client.get(&url).send()?;

        if res.status().is_success() {
            let index: FearAndGreedIndex = res.json()?;
            println!("[Debug] fear and greed index: {:?}", index);
            self.index = index
        } else {
            return Err(ClientError::CannotFetchDataFromAPIServer(url))?;
        }
        Ok(())
    }
}
