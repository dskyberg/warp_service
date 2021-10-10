use std::sync::Arc;
use warp::Filter;
use crate::{
    db::DB,
    cache::Cache,
    models::request::GrantOptions
};

#[derive(Clone)]
pub struct Service {
    pub db_client: DB,
    pub cache_client: Cache
}

impl Service {
    pub fn new(db_client: DB, cache_client: Cache) -> Self {
        Service { db_client, cache_client}
    }

    pub async fn get_dbs(&self) -> Vec<String> {
        self.db_client.list_databases().await.expect("bug!")
    }

    pub async fn get_grant_options(&self) -> GrantOptions {
        self.db_client.fetch_grant_options().await.expect("Could not load grant options")
    }
}

pub fn with_service(
    service: Arc<Service>,
) -> impl Filter<Extract = (Arc<Service>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}
