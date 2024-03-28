use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::fs::File;
use std::thread::sleep;
use client::prelude::*;
use anyhow::Result;
use std::io::Read;
use std::thread;
use std::time::Duration;
use tokio::runtime;
use tracing::Level;


fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let mut file = File::open("./config/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: ClientConfig = toml::from_str(&contents)?;
    println!("[Debug] {:?}", config);
    let db_conn = Conn::new()?;
    let mut feeder = Feeder::new(config.clone(), db_conn.clone())?;

    println!("[INFO] feeder created");

    prepare_earlier_data(&feeder)?;

    println!("[INFO] memory database prepared");

    let mut event_manager = EventManager::new(config.clone())?;

    println!("[INFO] event manager created");

    let config_clone = config.clone();

    thread::spawn(move || {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let server = serve(config_clone, db_conn.clone());

            println!("[INFO] web server listening..");

            server.await
        });
    });


    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("[INFO] quitting...");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Started feeding loop. Press Ctrl-C to terminate.");

    while running.load(Ordering::SeqCst) {
        match feeder.feed() {
            Ok(_) => println!("Feed successful"),
            Err(e) => eprintln!("Error occurred while feeding: {:?}", e),
        }
        println!("Event manager checking...");
        if let Err(e) = event_manager.check() {
            println!("An error occurred during event manager checking: {}", e);
        }
        println!("Event manager's job done");
        sleep(Duration::from_secs(feeder.caller.config.oracle.interval));
    }
    Ok(())

}


pub fn prepare_earlier_data (feeder: &Feeder) -> Result<()> {
    let latest_oracle_data = feeder.caller.get_oracle_data()?;
    feeder.db_conn.upsert(Some(&latest_oracle_data), None)?;
    let i = feeder.fetcher.fetch_fear_greed_indexes()?;
    let indexes = i.data;
    for index in indexes {
        let timestamp = index.timestamp.clone().parse::<u64>()?;
        if timestamp < latest_oracle_data.timestamp {
            let raw_data = index.value.parse::<u64>()?;
            let d =  FearAndGreedApiData{ raw_data, timestamp};
            feeder.db_conn.upsert(None, Some(d))?;
        }
    }
    Ok(())
}