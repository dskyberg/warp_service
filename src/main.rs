use dotenv::dotenv;
use std::sync::Arc;
use std::env;
use std::net::SocketAddr;

use warp::{self, Filter};

use log::{error, info, warn, LevelFilter};

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

#[macro_use]
extern crate serde_derive;

use crate::{
    db::DB,
    cache::Cache,
    services::{with_service, Service}
};

mod apis;
mod db;
mod cache;
mod handlers;
mod models;
mod routes;
mod serde_utils;
mod services;
mod errors;


/// Crate main.
/// The main service needs to be async, in order to leverage async services.
#[tokio::main]
async fn main() {
    // Load the values from `.env` into the environment.  Then we can use
    // normal std::env methods to access.
    dotenv().ok();

    // Fire up the logger.  Logging to stdout for now.  But this can be
    // easily changed and managed via log4j configs
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(log_config).unwrap();

     let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    // Create the db and cache instances.  This should really migrate to the
    // Service module.  But it works for now.
    let db_client = DB::new().await;
    let cache_client = Cache::new().await;
    let service = Arc::new(Service::new(db_client, cache_client));

    // Generate the routes collection.  to extend, just add more `.or(macro)` calls.
    let routes = grant_options!(Arc::clone(&service))
        .or(grant_post!(Arc::clone(&service)));

    // Start the service
    warp::serve(routes).run(api_address).await;
}
