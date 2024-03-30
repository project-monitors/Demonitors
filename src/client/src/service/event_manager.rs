use std::collections::BTreeMap;
use anyhow::{Result, anyhow};
use chrono::{DateTime, TimeDelta, Timelike, Utc};
use crate::prelude::{ClientConfig, EventCaller};



#[derive(Default, Debug)]
pub struct Event {
    pub open_ts: u64,
    pub close_ts: u64,
    pub opened: bool,
    pub fetched: bool,
    pub resolved: bool
}

pub struct EventManager {
    pub config: ClientConfig,
    pub event_caller: EventCaller,
    pub checklist: BTreeMap<u64, Event>,
}

impl EventManager {

    pub fn new(cfg: ClientConfig) -> Result<Self> {
        let config_clone = cfg.clone();
        let caller = EventCaller::new(cfg)?;
        let checklist = BTreeMap::new();
        Ok(Self{
            config: config_clone,
            event_caller: caller,
            checklist
        })
    }

    pub fn time_delta_add(origin_time: DateTime<Utc>, days: i64, hours: i64, seconds: i64) -> Result<u64> {
        Ok((origin_time +
            TimeDelta::try_days(days).ok_or(anyhow!("time delta error"))? +
            TimeDelta::try_hours(hours).ok_or(anyhow!("time delta error"))? +
            TimeDelta::try_seconds(seconds).ok_or(anyhow!("time delta error"))?).timestamp() as u64)
    }

    pub fn check(&mut self) -> Result<()>{
        self.hourly_create()?;
        self.daily_create()?;
        self.refine_and_resolve()?;
        Ok(())
    }

    pub fn hourly_create(&mut self) -> Result<()> {
        if self.config.market.hourly_create {
            let now: DateTime<Utc> = Utc::now();
            let now_rounded_hourly = now
                .with_minute(0)
                .and_then(|dt| dt.with_second(0))
                .and_then(|dt| dt.with_nanosecond(0))
                .ok_or(anyhow!("time rounded error"))?;

            let expected_hour_1 = Self::time_delta_add(now_rounded_hourly, 0, 0, 1)?;
            let expected_hour_2 = Self::time_delta_add(now_rounded_hourly, 0, 1, 1)?;
            let expected_hour_3 = Self::time_delta_add(now_rounded_hourly, 0, 2, 1)?;
            let expected_hours = [expected_hour_1, expected_hour_2, expected_hour_3];
            for &expected_hour in expected_hours.iter() {
                if !self.checklist.contains_key(&expected_hour) {
                    let close_ts = &expected_hour + 3600;
                    let open_ts = expected_hour;
                    if self.event_caller.fetch_event_market_data(close_ts).is_err() {
                        let sig = self.event_caller.create_event_market(
                            open_ts, close_ts, close_ts)?;
                        println!("[Debug] Create hourly event market successfully. Close ts: {} \n\
                    https://explorer.solana.com/tx/{}?cluster=devnet", close_ts, sig);
                    }

                    let now: DateTime<Utc> = Utc::now();
                    let check_ts = (now.timestamp() as u64).checked_add(3600).ok_or(anyhow!("ts add error"))?;
                    let mut toggled = false;
                    let mut fetched = false;
                    if check_ts > close_ts {
                        fetched = true;
                    }
                    let market = self.event_caller.fetch_event_market_data(close_ts)?;
                    if !market.is_opened {
                        toggled = true;
                    }
                    if toggled || fetched {
                        let sig = self.event_caller.toggle_event_market(
                            close_ts, toggled, fetched)?;
                        println!("[Debug] Toggle hourly event market successfully. Close ts: {}  \n\
                                https://explorer.solana.com/tx/{}?cluster=devnet", close_ts, sig);
                    }
                    let event = Event{
                        open_ts,
                        close_ts,
                        opened: true,
                        fetched,
                        ..Default::default()
                    };
                    self.checklist.insert(close_ts, event);
                }
            }
        }
        Ok(())
    }

    pub fn daily_create(&mut self) -> Result<()> {
        if self.config.market.daily_create {
            let now: DateTime<Utc> = Utc::now();
            let now_rounded_daily = now
                .with_hour(0)
                .and_then(|dt| dt.with_minute(0))
                .and_then(|dt| dt.with_second(0))
                .and_then(|dt| dt.with_nanosecond(0))
                .ok_or(anyhow!("time rounded error"))?;

            let expected_day_1 = Self::time_delta_add(now_rounded_daily, 0, 0, 0)?;
            let expected_day_2 = Self::time_delta_add(now_rounded_daily, 1, 0, 0)?;
            let expected_day_3 = Self::time_delta_add(now_rounded_daily, 2, 0, 0)?;
            let expected_days = [expected_day_1, expected_day_2, expected_day_3];
            for &expected_day in expected_days.iter() {
                if !self.checklist.contains_key(&expected_day) {
                    let close_ts = &expected_day + 86400;
                    let open_ts = expected_day;
                    if self.event_caller.fetch_event_market_data(close_ts).is_err() {
                        let sig = self.event_caller.create_event_market(
                            open_ts, close_ts, close_ts)?;
                        println!("[Debug] Create daily event market successfully. Close ts: {} \n\
                        https://explorer.solana.com/tx/{}?cluster=devnet", close_ts, sig);
                    }

                    let now: DateTime<Utc> = Utc::now();
                    let check_ts = (now.timestamp() as u64).checked_add(86400).ok_or(anyhow!("ts add error"))?;
                    let mut toggled = false;
                    let mut fetched = false;
                    if check_ts >= close_ts {
                        fetched = true;
                    }
                    let market = self.event_caller.fetch_event_market_data(close_ts)?;
                    if !market.is_opened {
                        toggled = true;
                    }
                    if toggled || fetched {
                        let sig = self.event_caller.toggle_event_market(
                            close_ts, toggled, fetched)?;
                        println!("[Debug] Toggle daily event market successfully. Close ts: {}  \n\
                            https://explorer.solana.com/tx/{}?cluster=devnet", close_ts, sig);
                    }
                    let event = Event{
                        open_ts,
                        close_ts,
                        opened: true,
                        fetched,
                        ..Default::default()
                    };
                    self.checklist.insert(close_ts, event);
                }
            }
        }
        Ok(())
    }

    pub fn refine_and_resolve(&mut self) -> Result<()> {
        for event in self.checklist.values_mut() {
            let now: DateTime<Utc> = Utc::now();
            let check_ts = now.timestamp() as u64;
            if check_ts > event.open_ts {
                if !event.fetched {
                    let event_market = self.event_caller.fetch_event_market_data(event.close_ts)?;
                    let mut toggle = true;
                    if event_market.is_opened {
                        toggle = false
                    }
                    let sig = self.event_caller.toggle_event_market(event.close_ts, toggle, true)?;
                    println!("[Debug][Refine] Toggle event market successfully. Close ts: {}  \n\
                            https://explorer.solana.com/tx/{}?cluster=devnet", event.close_ts, sig);
                    event.fetched = true;
                }
                if !event.resolved && check_ts > event.close_ts {
                    let prize =  self.config.tokenomics.event_prize
                        * 10_u64.pow(self.config.metadata.ft_token_decimal as u32);
                    let sig = self.event_caller.resolve(event.close_ts, prize)?;
                    println!("[Debug][Resolve] Resolve event market successfully. Close ts: {}  \n\
                            https://explorer.solana.com/tx/{}?cluster=devnet",  event.close_ts, sig);
                    event.resolved = true;
                }
            }
        }

        self.checklist.retain(|_, v| !v.resolved);

        Ok(())
    }
}