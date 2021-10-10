use std::sync::Arc;
use warp::Filter;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client, FromRedisValue, Value};
use log::{debug};

use crate::{
    db::DB,
    cache::Cache,
    models::request::GrantOptions,
    errors::Error
};

#[derive(Clone)]
pub struct Service {
    pub db_client: DB,
    pub cache_client: Cache
}
const CACHE_KEY_PREFIX: &str = "gnap";

impl Service {
    pub fn new(db_client: DB, cache_client: Cache) -> Self {
        Service { db_client, cache_client}
    }

    pub async fn get_dbs(&self) -> Vec<String> {
        self.db_client.list_databases().await.expect("bug!")
    }

    pub async fn get_grant_options(&self) -> Result<GrantOptions, Error> {
        // self.db_client.fetch_grant_options().await.expect("Could not load grant options")
        let cache_key = "gnap:grant_options";
        let mut con = self.cache_client.client.get_async_connection().await?;
        let cache_response = con.get(cache_key).await?;

        match cache_response {
            Value::Nil => {
                debug!("Use database to retrieve GranOptions");
                let result = self.db_client.fetch_grant_options().await?;

                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, &result)
                    .expire(&cache_key, 60)
                    .query_async(&mut con)
                    .await?;

                Ok(result)
            }
            Value::Data(val) => {
                debug!("Use cache to retrieve GrantOptions");
                Ok(serde_json::from_slice(&val)?)
            }
            _ => {
                debug!("Did not successfully get a cache response");
                Err(Error::GeneralError)
            }
        }
    }


}

pub fn with_service(
    service: Arc<Service>,
) -> impl Filter<Extract = (Arc<Service>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}
