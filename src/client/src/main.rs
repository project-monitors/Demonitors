use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::fs::File;
use std::thread::sleep;
use client::prelude::*;
use anyhow::Result;
use std::io::Read;
use std::time::Duration;


fn main() -> Result<()> {
    let mut file = File::open("./config/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: ClientConfig = toml::from_str(&contents)?;
    println!("[Debug] {:?}", config);
    let config: Arc<ClientConfig> = Arc::new(config);
    let mut feeder = Feeder::new(config)?;

    println!("[INFO] feeder created");

    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Started feeding loop. Press Ctrl-C to terminate.");

    while running.load(Ordering::SeqCst) {
        match feeder.feed() {
            Ok(_) => println!("Feed successful"),
            Err(e) => eprintln!("Error occurred while feeding: {:?}", e),
        }

        sleep(Duration::from_secs(feeder.caller.config.oracle.interval));
    }
    Ok(())

}
