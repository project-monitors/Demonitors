use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("The param '{0}' is not valid.")]
    InvalidParam(String),
}
