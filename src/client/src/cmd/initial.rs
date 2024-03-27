use anyhow::Error;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use client::prelude::*;
use factory::instructions::CreateEventSBTParams;


fn main() {
    match initialize() {
        Ok(_) => {
            println!("[INFO] Initial accounts successfully.");
        }
        Err(e) => {
            println!("[ERROR] Initial accounts failed, error: {}", e)
        }
    }
}

fn initialize () -> Result<(), Error> {
    let file = File::open("./config/config.toml");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents)?;
    let config: ClientConfig = toml::from_str(&contents)?;
    println!("{:?}", config);
    let config: Arc<ClientConfig> = Arc::new(config);
    let mut suffix = "";
    if config.solana.network == "devnet" {
        suffix = "?cluster=devnet";
    } else if config.solana.network == "testnet" {
        suffix = "?cluster=testnet";
    }

    let config_clone = config.clone();

    let caller = ChainCaller::new(config)?;

    let sig = caller.initialize_oracle_config()?;
    //https://explorer.solana.com/tx/4wRAytwajYcWrV7pHuV5MuNaLAeis6zrKLycUT2iGJqTbs3QxfQbtuTChuEhqsxH2FJuXbxJnkavAvgKomeoETiG?cluster=devnet
    println!("[Debug] Initial oracle config successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let sig = caller.initialize_oracle_data()?;
    println!("[Debug] Initial oracle data successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let oracle_config = caller.get_oracle_config()?;
    println!("[Debug] Oracle config name is {:?}", oracle_config.name);

    let  data = caller.get_oracle_data()?;
    println!("[Debug] Oracle data raw data is {:?}", data.raw_data);

    let event_caller = EventCaller::new(config_clone)?;

    let sig = event_caller.initialize_global_config()?;
    println!("[Debug] Initial global config successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let sig = event_caller.initialize_mint()?;
    println!("[Debug] Initial mint successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let mint_to_vision_mining = event_caller.config.tokenomics.vision_mining;
    let mint_to_event_mining = event_caller.config.tokenomics.event_mining;

    let vision_mining_pda = event_caller.get_const_name_pda(EventCaller::VISION_MINING_SEED)?;
    let sig = event_caller.mint_tokens(vision_mining_pda, mint_to_vision_mining)?;
    println!("[Debug] Mint to vision_mining_pda successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let event_mining_pda = event_caller.get_const_name_pda(EventCaller::EVENT_MINING_SEED)?;
    let sig = event_caller.mint_tokens(event_mining_pda, mint_to_event_mining)?;
    println!("[Debug] Mint to event_mining_pda successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let sig = event_caller.initialize_collection()?;
    println!("[Debug] Initialize collection successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let option: u8 = 2;
    let meta_json_url = event_caller.config.metadata.event_metadata_url.clone();

    let sig = event_caller.create_event_config(option, meta_json_url)?;
    println!("[Debug] Create event config successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let fear = CreateEventSBTParams{
        name: event_caller.config.metadata.fear_sbt_token_name.to_string(),
        symbol: event_caller.config.metadata.fear_sbt_token_symbol.to_string(),
        uri: event_caller.config.metadata.fear_sbt_token_url.to_string(),
        option: 1,
    };
    let sig = event_caller.create_event_sbt(fear)?;
    println!("[Debug] Create fear sbt successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);
    let sig = event_caller.mint_event_sbt_master_edition(1)?;
    println!("[Debug] Mint fear sbt master edition successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let greed = CreateEventSBTParams{
        name: event_caller.config.metadata.greed_sbt_token_name.to_string(),
        symbol: event_caller.config.metadata.greed_sbt_token_symbol.to_string(),
        uri: event_caller.config.metadata.greed_sbt_token_url.to_string(),
        option: 2,
    };
    let sig = event_caller.create_event_sbt(greed)?;
    println!("[Debug] Create greed sbt successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);
    let sig = event_caller.mint_event_sbt_master_edition(2)?;
    println!("[Debug] Mint greed sbt master edition successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    Ok(())
}


