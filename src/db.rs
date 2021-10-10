
//! Database abstraction layer
//!
//! Ideally, there should be some traits established that are implemented for
//! MongoDB.  The traits would then be a contract for use by the handlers that
//! interact with data.

use crate::models::request::GrantOptions;
//use futures::stream::{self, StreamExt, TryStreamExt};
use crate::errors::Error;
use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub db: Database
}

//const MONGO_URI: &str = "mongodb://127.0.0.1:27017";

impl DB {
    pub async fn new() -> Self {
        // Read the config from either the environment or a .env file.
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing");
        let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
        let app_name = env::var("MONGODB_APP_NAME").expect("MONGODB_APP_NAME missing");

        // Create the ClientOptions and set the app_name
        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        // Create the client and grab a database handle
        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
        let d = client.database(&database);
        DB {
            client: client,
            db: d
        }
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, Error> {
        match self.client.list_database_names(None, None).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::MongoError(e)),
        }
    }

    pub async fn fetch_grant_options(&self) -> Result<GrantOptions, Error> {
        let mut cursor = self
            .db
            .collection::<GrantOptions>("grant_options")
            .find(None, None)
            .await
            .map_err(Error::MongoQueryError)?;

        match cursor.try_next().await {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Ok(GrantOptions::new()),
            Err(e) => Err(Error::MongoQueryError(e)),
        }
    }
}
