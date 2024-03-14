use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use client::prelude::*;
use anyhow::Result;
use tokio::task::block_in_place;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    let mut file = File::open("./config/config.toml").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: ClientConfig = toml::from_str(&contents)?;
    println!("[Debug] {:?}", config);
    let config: Arc<ClientConfig> = Arc::new(config);
    let feeder = block_in_place(||Feeder::new(config).unwrap_or_else(|err| {
        eprintln!("Failed to create Feeder: {:?}", err);
        std::process::exit(1);
        }));
    let feeder = Arc::new(Mutex::new(feeder));
    println!("[INFO] feeder created {:?}", feeder.lock().await.caller.payer);

    let sched = JobScheduler::new().await;
    let mut sched = sched.unwrap();
    let jobs = run_oracle_feeder(&mut sched, feeder)
        .await
        .expect("Could not run example");
    stop_oracle_feeder(&mut sched, jobs)
        .await
        .expect("Could not stop example");

    Ok(())

    // println!("[Debug] help me please!");
    // let feeder_job = Job::new_async("* 1 * * * *", move |_uuid, _l| {
    //     let feeder_clone = feeder.clone();
    //     println!("[Debug] help me!");
    //     Box::pin(
    //         async move {
    //             let mut feeder_clone = feeder_clone.lock().await;
    //             match feeder_clone.feed().await {
    //                 Ok(_) => (),
    //                 Err(e) => eprintln!("Error occurred while feeding: {:?}", e),
    //             }
    //         })
    // }).unwrap();
    //
    // sched.add(feeder_job).await?;
    //
    // sched.start().await?;
    //
    // Ok(())

}
