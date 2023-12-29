use super::mongo::Mongo;
use crate::DarthTools;
use async_trait::async_trait;
use mongodb::{error::Error, Client};

#[async_trait]
pub trait MongodbTrait {
    async fn mongodb_connect<Item>(
        endpoint: &str,
        db: &str,
        table: &str,
    ) -> Result<Mongo<Item>, Error>
    where
        Item: for<'de> serde::Deserialize<'de> + serde::Serialize;
}

#[async_trait]
impl MongodbTrait for DarthTools {
    async fn mongodb_connect<
        Item: for<'de> serde::Deserialize<'de> + serde::Serialize,
    >(
        endpoint: &str,
        db: &str,
        table: &str,
    ) -> Result<Mongo<Item>, Error> {
        let client = Client::with_uri_str(endpoint).await?;
        let database = client.database(db);
        let collection = database.collection::<Item>(table);
        Ok(Mongo { collection, database, client })
    }
}
