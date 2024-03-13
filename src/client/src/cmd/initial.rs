use anyhow::Error;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use client::prelude::*;



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

    let caller = ChainCaller::new(config)?;

    let sig = caller.initialize_oracle_config()?;
    //https://explorer.solana.com/tx/4wRAytwajYcWrV7pHuV5MuNaLAeis6zrKLycUT2iGJqTbs3QxfQbtuTChuEhqsxH2FJuXbxJnkavAvgKomeoETiG?cluster=devnet
    println!("[Debug] Initial oracle config successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    let sig = caller.initialize_oracle_data()?;
    println!("[Debug] Initial oracle data successfully. \n\
        https://explorer.solana.com/tx/{}{}", sig, suffix);

    Ok(())
}


