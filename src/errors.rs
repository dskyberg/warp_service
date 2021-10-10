use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    //    #[error("invalid id used: {0}")]
    //    InvalidIDError(String),
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("General error")]
    GeneralError
}

impl From<serde_json::Error> for Error {
    fn from(_source: serde_json::Error) -> Self {
        Self::GeneralError
    }
}

impl warp::reject::Reject for Error {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
