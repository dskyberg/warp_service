use redis::aio::ConnectionManager;
use redis::Client;
use redis::{RedisWrite, ToRedisArgs};
use std::env;

use crate::models::request;

#[derive(Clone)]
pub struct Cache {
    pub client: Client,
    pub connection_manager: ConnectionManager,
}

impl Cache {
    pub async fn new() -> Self {
        let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");

        let client = Client::open(redis_uri).expect("Failed to open Redis client");

        let connection_manager = client
            .get_tokio_connection_manager()
            .await
            .expect("Can't create Redis connection manager");

        Cache {
            client,
            connection_manager,
        }
    }
}

impl ToRedisArgs for &request::GrantOptions {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize GrantOptions as string"))
    }
}
