use mongodb::{
    bson::Document,
    error::Error,
    options::{
        AggregateOptions, CreateIndexOptions,
        CreateSearchIndexOptions, DeleteOptions,
        DropCollectionOptions, FindOptions, InsertManyOptions,
        InsertOneOptions, SessionOptions, UpdateOptions,
        UpdateSearchIndexOptions,
    },
    results::{
        CreateIndexResult, CreateIndexesResult, DeleteResult,
        InsertManyResult, InsertOneResult, UpdateResult,
    },
    Client, Collection, Cursor, Database, IndexModel,
    SearchIndexModel, SessionCursor,
};
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

#[derive(Debug)]
pub struct Mongo<Item: Serialize + Deserialize<'static>> {
    pub collection: Collection<Item>,
    pub database: Database,
    pub client: Client,
}

impl<Item: Serialize + Deserialize<'static>> Mongo<Item> {
    pub async fn aggregate_with_session(
        &self,
        secs: u64,
        filter: Vec<Document>,
        options_aggregate: Option<AggregateOptions>,
        options_session: Option<SessionOptions>,
    ) -> Result<SessionCursor<Document>, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(options_session).await?;
        let future = collection.aggregate_with_session(
            filter,
            options_aggregate,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn aggregate(
        &self,
        secs: u64,
        filter: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Cursor<Document>, Error> {
        let collection = &self.collection;
        let future = collection.aggregate(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_index(
        &self,
        secs: u64,
        filter: IndexModel,
        options: Option<CreateIndexOptions>,
    ) -> Result<CreateIndexResult, Error> {
        let collection = &self.collection;
        let future = collection.create_index(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_search_index(
        &self,
        secs: u64,
        filter: SearchIndexModel,
        options: Option<CreateSearchIndexOptions>,
    ) -> Result<String, Error> {
        let collection = &self.collection;
        let future = collection.create_search_index(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_search_indexes(
        &self,
        secs: u64,
        filter: Vec<SearchIndexModel>,
        options: Option<CreateSearchIndexOptions>,
    ) -> Result<Vec<String>, Error> {
        let collection = &self.collection;
        let future =
            collection.create_search_indexes(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_indexes(
        &self,
        secs: u64,
        filter: Vec<IndexModel>,
        options: Option<CreateIndexOptions>,
    ) -> Result<CreateIndexesResult, Error> {
        let collection = &self.collection;
        let future = collection.create_indexes(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_index_with_session(
        &self,
        secs: u64,
        filter: IndexModel,
        index_options: Option<CreateIndexOptions>,
        session_options: SessionOptions,
    ) -> Result<CreateIndexResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.create_index_with_session(
            filter,
            index_options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn create_indexes_with_session(
        &self,
        secs: u64,
        filter: Vec<IndexModel>,
        index_options: Option<CreateIndexOptions>,
        session_options: SessionOptions,
    ) -> Result<CreateIndexesResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.create_indexes_with_session(
            filter,
            index_options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn find(
        &self,
        secs: u64,
        filter: Document,
        options: Option<FindOptions>,
    ) -> Result<Cursor<Item>, Error> {
        let collection = &self.collection;
        let future = collection.find(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn delete_many_with_session(
        &self,
        secs: u64,
        filter: Document,
        delete_options: Option<DeleteOptions>,
        session_options: SessionOptions,
    ) -> Result<DeleteResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.delete_many_with_session(
            filter,
            delete_options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn delete_one_with_session(
        &self,
        secs: u64,
        filter: Document,
        delete_options: Option<DeleteOptions>,
        session_options: SessionOptions,
    ) -> Result<DeleteResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.delete_one_with_session(
            filter,
            delete_options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn delete_many(
        &self,
        secs: u64,
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> Result<DeleteResult, Error> {
        let collection = &self.collection;
        let future = collection.delete_many(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn delete_one(
        &self,
        secs: u64,
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> Result<DeleteResult, Error> {
        let collection = &self.collection;
        let future = collection.delete_one(filter, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn update_many(
        &self,
        secs: u64,
        filter: Document,
        options: Option<UpdateOptions>,
        update: Document,
    ) -> Result<UpdateResult, Error> {
        let collection = &self.collection;
        let future = collection.update_many(filter, update, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn update_many_with_session(
        &self,
        secs: u64,
        filter: Document,
        options_update: Option<UpdateOptions>,
        session_options: SessionOptions,
        update: Document,
    ) -> Result<UpdateResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.update_many_with_session(
            filter,
            update,
            options_update,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn update_one_with_session(
        &self,
        secs: u64,
        filter: Document,
        options_update: Option<UpdateOptions>,
        session_options: SessionOptions,
        update: Document,
    ) -> Result<UpdateResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.update_one_with_session(
            filter,
            update,
            options_update,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn update_one(
        &self,
        secs: u64,
        filter: Document,
        options: Option<UpdateOptions>,
        update: Document,
    ) -> Result<UpdateResult, Error> {
        let collection = &self.collection;
        let future = collection.update_one(filter, update, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn drop(
        &self,
        secs: u64,
        options: Option<DropCollectionOptions>,
    ) -> Result<(), Error> {
        let collection = &self.collection;
        let future = collection.drop(options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn update_search_index(
        &self,
        secs: u64,
        name: &str,
        definition: Document,
        options: Option<UpdateSearchIndexOptions>,
    ) -> Result<(), Error> {
        let collection = &self.collection;
        let future =
            collection.update_search_index(name, definition, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn insert_one(
        &self,
        secs: u64,
        doc: Item,
        options: Option<InsertOneOptions>,
    ) -> Result<InsertOneResult, Error> {
        let collection = &self.collection;
        let future = collection.insert_one(doc, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn insert_one_with_session(
        &self,
        secs: u64,
        doc: Item,
        options: Option<InsertOneOptions>,
        session_options: SessionOptions,
    ) -> Result<InsertOneResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.insert_one_with_session(
            doc,
            options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn insert_many_with_session(
        &self,
        secs: u64,
        doc: Vec<Item>,
        options: Option<InsertManyOptions>,
        session_options: SessionOptions,
    ) -> Result<InsertManyResult, Error> {
        let collection = &self.collection;
        let mut session =
            self.client.start_session(session_options).await?;
        let future = collection.insert_many_with_session(
            doc,
            options,
            &mut session,
        );
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
    pub async fn insert_many(
        &self,
        secs: u64,
        doc: Vec<Item>,
        options: Option<InsertManyOptions>,
    ) -> Result<InsertManyResult, Error> {
        let collection = &self.collection;
        let future = collection.insert_many(doc, options);
        match timeout(Duration::from_secs(secs), future).await {
            Ok(ok) => ok,
            Err(err) => Err(Error::custom(err)),
        }
    }
}
