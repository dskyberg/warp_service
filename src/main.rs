#[macro_use]
extern crate serde_derive;

use crate::db::DB;
use crate::services::{with_service, Service};
use dotenv::dotenv;
use std::sync::Arc;
use warp::{self, Filter};

mod apis;
mod db;
mod handlers;
mod models;
mod routes;
mod serde_utils;
mod services;
mod errors;

// type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongodb_client = DB::new().await;
    let service = Arc::new(Service::new(mongodb_client));

    let routes = grant_options!(Arc::clone(&service))
        .or(grant_post!(Arc::clone(&service)));

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
