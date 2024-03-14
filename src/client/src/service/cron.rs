use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;
use tracing::{error, info, warn};
use crate::prelude::Feeder;



pub async fn run_oracle_feeder(sched: &mut JobScheduler, feeder: Arc<Mutex<Feeder>>) -> Result<Vec<Uuid>, JobSchedulerError> {

    // let mut four_s_job_async = Job::new_async_tz("1/4 * * * * *", Utc, |uuid, mut l| {
    //     Box::pin(async move {
    //         info!("I run async every 4 seconds id {:?}", uuid);
    //         let next_tick = l.next_tick_for_job(uuid).await;
    //         match next_tick {
    //             Ok(Some(ts)) => info!("Next time for 4s is {:?}", ts),
    //             _ => warn!("Could not get next tick for 4s job"),
    //         }
    //     })
    // })
    //     .unwrap();
    // let four_s_job_async_clone = four_s_job_async.clone();
    // let js = sched.clone();
    // info!("4s job id {:?}", four_s_job_async.guid());
    // four_s_job_async.on_start_notification_add(&sched, Box::new(move |job_id, notification_id, type_of_notification| {
    //     let four_s_job_async_clone = four_s_job_async_clone.clone();
    //     let js = js.clone();
    //     Box::pin(async move {
    //         info!("4s Job {:?} ran on start notification {:?} ({:?})", job_id, notification_id, type_of_notification);
    //         info!("This should only run once since we're going to remove this notification immediately.");
    //         info!("Removed? {:?}", four_s_job_async_clone.on_start_notification_remove(&js, &notification_id).await);
    //     })
    // })).await?;
    //
    // four_s_job_async
    //     .on_done_notification_add(
    //         &sched,
    //         Box::new(|job_id, notification_id, type_of_notification| {
    //             Box::pin(async move {
    //                 info!(
    //                     "4s Job {:?} completed and ran notification {:?} ({:?})",
    //                     job_id, notification_id, type_of_notification
    //                 );
    //             })
    //         }),
    //     )
    //     .await?;
    //
    // let four_s_job_guid = four_s_job_async.guid();
    // sched.add(four_s_job_async).await?;

    println!("bbbbbb");
    let mut oracle_feeder_job = Job::new_async_tz(
        "1/10 * * * * *", Utc, move |uuid, mut l| {
            println!("aaaaaa");
            let feeder_clone = feeder.clone();
            Box::pin(async move {
                // info!("I run async every 4 seconds id {:?}", uuid);
                // let next_tick = l.next_tick_for_job(uuid).await;
                // match next_tick {
                //     Ok(Some(ts)) => info!("Next time for 4s is {:?}", ts),
                //     _ => warn!("Could not get next tick for 4s job"),
                // }
                println!("hahahah");
                let mut feeder_clone = feeder_clone.lock().await;
                match feeder_clone.feed().await {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error occurred while feeding: {:?}", e),
                }
            })
        })
        .unwrap();

    let oracle_feeder_job_guid = oracle_feeder_job.guid();
    sched.add(oracle_feeder_job).await?;

    let start = sched.start().await;

    let ret = vec![];

    Ok(ret)

    // #[cfg(feature = "signal")]
    // sched.shutdown_on_ctrl_c();

    // sched.set_shutdown_handler(Box::new(|| {
    //     Box::pin(async move {
    //         info!("Shut down done");
    //     })
    // }));
    //
    // let mut five_s_job = Job::new("1/5 * * * * *", |uuid, _l| {
    //     info!(
    //         "{:?} I run every 5 seconds id {:?}",
    //         chrono::Utc::now(),
    //         uuid
    //     );
    // })
    //     .unwrap();
    //
    // // Adding a job notification without it being added to the scheduler will automatically add it to
    // // the job store, but with stopped marking
    // five_s_job
    //     .on_removed_notification_add(
    //         &sched,
    //         Box::new(|job_id, notification_id, type_of_notification| {
    //             Box::pin(async move {
    //                 info!(
    //                     "5s Job {:?} was removed, notification {:?} ran ({:?})",
    //                     job_id, notification_id, type_of_notification
    //                 );
    //             })
    //         }),
    //     )
    //     .await?;
    // let five_s_job_guid = five_s_job.guid();
    // sched.add(five_s_job).await?;
    //
    // let mut four_s_job_async = Job::new_async_tz("1/4 * * * * *", Utc, |uuid, mut l| {
    //     Box::pin(async move {
    //         info!("I run async every 4 seconds id {:?}", uuid);
    //         let next_tick = l.next_tick_for_job(uuid).await;
    //         match next_tick {
    //             Ok(Some(ts)) => info!("Next time for 4s is {:?}", ts),
    //             _ => warn!("Could not get next tick for 4s job"),
    //         }
    //     })
    // })
    //     .unwrap();
    // let four_s_job_async_clone = four_s_job_async.clone();
    // let js = sched.clone();
    // info!("4s job id {:?}", four_s_job_async.guid());
    // four_s_job_async.on_start_notification_add(&sched, Box::new(move |job_id, notification_id, type_of_notification| {
    //     let four_s_job_async_clone = four_s_job_async_clone.clone();
    //     let js = js.clone();
    //     Box::pin(async move {
    //         info!("4s Job {:?} ran on start notification {:?} ({:?})", job_id, notification_id, type_of_notification);
    //         info!("This should only run once since we're going to remove this notification immediately.");
    //         info!("Removed? {:?}", four_s_job_async_clone.on_start_notification_remove(&js, &notification_id).await);
    //     })
    // })).await?;
    //
    // four_s_job_async
    //     .on_done_notification_add(
    //         &sched,
    //         Box::new(|job_id, notification_id, type_of_notification| {
    //             Box::pin(async move {
    //                 info!(
    //                     "4s Job {:?} completed and ran notification {:?} ({:?})",
    //                     job_id, notification_id, type_of_notification
    //                 );
    //             })
    //         }),
    //     )
    //     .await?;
    //
    // let four_s_job_guid = four_s_job_async.guid();
    // sched.add(four_s_job_async).await?;
    //
    // sched
    //     .add(
    //         Job::new("1/30 * * * * *", |uuid, _l| {
    //             info!("I run every 30 seconds id {:?}", uuid);
    //         })
    //             .unwrap(),
    //     )
    //     .await?;
    //
    // info!(
    //     "Sched one shot for {:?}",
    //     chrono::Utc::now()
    //         .checked_add_signed(chrono::Duration::seconds(10))
    //         .unwrap()
    // );
    // sched
    //     .add(
    //         Job::new_one_shot(Duration::from_secs(10), |_uuid, _l| {
    //             info!("I'm only run once");
    //         })
    //             .unwrap(),
    //     )
    //     .await?;
    //
    // info!(
    //     "Sched one shot async for {:?}",
    //     chrono::Utc::now()
    //         .checked_add_signed(chrono::Duration::seconds(16))
    //         .unwrap()
    // );
    // sched
    //     .add(
    //         Job::new_one_shot_async(Duration::from_secs(16), |_uuid, _l| {
    //             Box::pin(async move {
    //                 info!("I'm only run once async");
    //             })
    //         })
    //             .unwrap(),
    //     )
    //     .await?;
    //
    // let jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
    //     info!("I'm repeated every 8 seconds");
    // })
    //     .unwrap();
    // let jj_guid = jj.guid();
    // sched.add(jj).await?;
    //
    // let jja = Job::new_repeated_async(Duration::from_secs(7), |_uuid, _l| {
    //     Box::pin(async move {
    //         info!("I'm repeated async every 7 seconds");
    //     })
    // })
    //     .unwrap();
    // let jja_guid = jja.guid();
    // sched.add(jja).await?;
    //
    // let utc_job = JobBuilder::new()
    //     .with_timezone(Utc)
    //     .with_cron_job_type()
    //     .with_schedule("*/2 * * * * *")
    //     .unwrap()
    //     .with_run_async(Box::new(|uuid, mut l| {
    //         Box::pin(async move {
    //             info!("UTC run async every 2 seconds id {:?}", uuid);
    //             let next_tick = l.next_tick_for_job(uuid).await;
    //             match next_tick {
    //                 Ok(Some(ts)) => info!("Next time for UTC 2s is {:?}", ts),
    //                 _ => warn!("Could not get next tick for 2s job"),
    //             }
    //         })
    //     }))
    //     .build()
    //     .unwrap();
    //
    // let utc_job_guid = utc_job.guid();
    // sched.add(utc_job).await.unwrap();
    //
    // let jhb_job = JobBuilder::new()
    //     .with_timezone(chrono_tz::Africa::Johannesburg)
    //     .with_cron_job_type()
    //     .with_schedule("*/2 * * * * *")
    //     .unwrap()
    //     .with_run_async(Box::new(|uuid, mut l| {
    //         Box::pin(async move {
    //             info!("JHB run async every 2 seconds id {:?}", uuid);
    //             let next_tick = l.next_tick_for_job(uuid).await;
    //             match next_tick {
    //                 Ok(Some(ts)) => info!("Next time for JHB 2s is {:?}", ts),
    //                 _ => warn!("Could not get next tick for 2s job"),
    //             }
    //         })
    //     }))
    //     .build()
    //     .unwrap();
    //
    // let jhb_job_guid = jhb_job.guid();
    // sched.add(jhb_job).await.unwrap();
    //
    // let start = sched.start().await;
    // if let Err(e) = start {
    //     error!("Error starting scheduler {}", e);
    //     return Err(e);
    // }
    //
    // let ret = vec![
    //     five_s_job_guid,
    //     four_s_job_guid,
    //     jj_guid,
    //     jja_guid,
    //     utc_job_guid,
    //     jhb_job_guid,
    // ];
    // return Ok(ret);

}

pub async fn stop_oracle_feeder(
    sched: &mut JobScheduler,
    jobs: Vec<Uuid>,
) -> Result<(), JobSchedulerError> {
    tokio::time::sleep(Duration::from_secs(20)).await;

    for i in jobs {
        sched.remove(&i).await?;
    }

    tokio::time::sleep(Duration::from_secs(40)).await;

    info!("Goodbye.");
    sched.shutdown().await?;
    Ok(())
}