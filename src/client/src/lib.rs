mod chain;
mod core;
mod service;
mod util;
mod data;
mod api;

pub mod prelude {
    pub use crate::core::conf::ClientConfig;
    pub use crate::chain::oracle::{
        ChainCaller,
        OracleDataRequest,
    };
    pub use crate::chain::event::EventCaller;
    pub use crate::service::{
        feeder::Feeder,
        event_manager::EventManager
    };
    pub use crate::data::mem_db::{Conn, FearAndGreedApiData};
    pub use crate::api::serve;
}
