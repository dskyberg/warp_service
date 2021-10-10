use warp;
use std::sync::Arc;
use crate::services::Service;
use crate::models::request::{GrantRequest, GrantOptions};

pub async fn grant_options(service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {

    //let options = GrantOptions::new();
    let options:GrantOptions = service.get_grant_options().await?;
    Ok(warp::reply::json(&options))
}

pub async fn grant_post(request: GrantRequest, service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&request))
}