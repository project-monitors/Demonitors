mod chain;
mod core;

pub mod prelude {
    pub use crate::core::conf::ClientConfig;
    pub use crate::chain::oracle::{
        ChainCaller,
        OracleDataRequest,
    };
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
