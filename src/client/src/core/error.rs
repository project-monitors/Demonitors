use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("The param '{0}' is not valid.")]
    InvalidParam(String),
    #[error("Can not fetch data from API server '{0}'")]
    CannotFetchDataFromAPIServer(String),
    #[error("Can not find account from Solana blockchain")]
    CannotFindAccountFromBlockChain,
    #[error("Can not deserialize account from account data")]
    CannotDeserializeAccountData
}
