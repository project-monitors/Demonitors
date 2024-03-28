use axum::{
    extract::{Path, State},
};
use super::AppState;
use super::response::*;
use crate::data::mem_db::OracleDataFetched;


pub async fn get_oracle_data(
    State(state): State<AppState>,
    Path(timestamp): Path<u64>,
) -> Result<ApiOK<OracleDataFetched>> {
    match state.db.fetch_oracle_data(timestamp)
    {
        Err(err) => {
            tracing::error!(error = ?err, "err get data from memory db");
            Err(ApiErr::ErrSystem(None))
    }
        Ok(v) => {
            Ok(ApiOK(Some(v)))
        }
    }
}