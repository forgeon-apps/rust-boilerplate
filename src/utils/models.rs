#![allow(dead_code)]

use async_trait::async_trait;
use futures::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};
use validator::Validate;

use wither::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    mongodb::{
        options::{
            FindOneAndUpdateOptions, FindOneOptions, FindOptions, ReturnDocument, UpdateOptions,
        },
        results::{DeleteResult, UpdateResult},
    },
    Model as WitherModel,
    ModelCursor,
};

use crate::{database, errors::Error};

// This is the Model trait. All models that have a MongoDB collection should
// implement this and therefore inherit these methods.
//
// Enterprise note:
// async_trait produces Send futures by default → we require Send + Sync here.
#[async_trait]
pub trait ModelExt: WitherModel + Validate + Sized + Send + Sync {
    async fn create(mut model: Self) -> Result<Self, Error> {
        let connection = database::connection().await;
        model.validate().map_err(|_| Error::bad_request())?;
        model.save(connection, None).await.map_err(Error::Wither)?;
        Ok(model)
    }

    async fn find_by_id(id: &ObjectId) -> Result<Option<Self>, Error> {
        let connection = database::connection().await;
        <Self as WitherModel>::find_one(connection, doc! { "_id": id }, None)
            .await
            .map_err(Error::Wither)
    }

    async fn find_one<O>(query: Document, options: O) -> Result<Option<Self>, Error>
    where
        O: Into<Option<FindOneOptions>> + Send,
    {
        let connection = database::connection().await;
        <Self as WitherModel>::find_one(connection, query, options)
            .await
            .map_err(Error::Wither)
    }

    async fn find<O>(query: Document, options: O) -> Result<Vec<Self>, Error>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        let connection = database::connection().await;
        <Self as WitherModel>::find(connection, query, options)
            .await
            .map_err(Error::Wither)?
            .try_collect::<Vec<Self>>()
            .await
            .map_err(Error::Wither)
    }

    async fn find_and_count<O>(query: Document, options: O) -> Result<(Vec<Self>, u64), Error>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        let connection = database::connection().await;

        let count = Self::collection(connection)
            .count_documents(query.clone(), None)
            .await
            .map_err(Error::Mongo)?;

        let items = <Self as WitherModel>::find(connection, query, options.into())
            .await
            .map_err(Error::Wither)?
            .try_collect::<Vec<Self>>()
            .await
            .map_err(Error::Wither)?;

        Ok((items, count))
    }

    async fn cursor<O>(query: Document, options: O) -> Result<ModelCursor<Self>, Error>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        let connection = database::connection().await;
        <Self as WitherModel>::find(connection, query, options)
            .await
            .map_err(Error::Wither)
    }

    async fn find_one_and_update(query: Document, update: Document) -> Result<Option<Self>, Error> {
        let connection = database::connection().await;
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        <Self as WitherModel>::find_one_and_update(connection, query, update, options)
            .await
            .map_err(Error::Wither)
    }

    async fn update_one<O>(
        query: Document,
        update: Document,
        options: O,
    ) -> Result<UpdateResult, Error>
    where
        O: Into<Option<UpdateOptions>> + Send,
    {
        let connection = database::connection().await;
        Self::collection(connection)
            .update_one(query, update, options)
            .await
            .map_err(Error::Mongo)
    }

    async fn update_many<O>(
        query: Document,
        update: Document,
        options: O,
    ) -> Result<UpdateResult, Error>
    where
        O: Into<Option<UpdateOptions>> + Send,
    {
        let connection = database::connection().await;
        Self::collection(connection)
            .update_many(query, update, options)
            .await
            .map_err(Error::Mongo)
    }

    async fn delete_many(query: Document) -> Result<DeleteResult, Error> {
        let connection = database::connection().await;
        <Self as WitherModel>::delete_many(connection, query, None)
            .await
            .map_err(Error::Wither)
    }

    async fn delete_one(query: Document) -> Result<DeleteResult, Error> {
        let connection = database::connection().await;
        Self::collection(connection)
            .delete_one(query, None)
            .await
            .map_err(Error::Mongo)
    }

    async fn count(query: Document) -> Result<u64, Error> {
        let connection = database::connection().await;
        Self::collection(connection)
            .count_documents(query, None)
            .await
            .map_err(Error::Mongo)
    }

    async fn exists(query: Document) -> Result<bool, Error> {
        let connection = database::connection().await;
        let count = Self::collection(connection)
            .count_documents(query, None)
            .await
            .map_err(Error::Mongo)?;
        Ok(count > 0)
    }

    async fn aggregate<A>(pipeline: Vec<Document>) -> Result<Vec<A>, Error>
    where
        A: Serialize + DeserializeOwned + Send,
    {
        let connection = database::connection().await;

        let documents = Self::collection(connection)
            .aggregate(pipeline, None)
            .await
            .map_err(Error::Mongo)?
            .try_collect::<Vec<Document>>()
            .await
            .map_err(Error::Mongo)?;

        let documents = documents
            .into_iter()
            .map(|document| bson::from_bson::<A>(Bson::Document(document)))
            .collect::<Result<Vec<A>, bson::de::Error>>()
            .map_err(Error::SerializeMongoResponse)?;

        Ok(documents)
    }

    async fn sync_indexes() -> Result<(), Error> {
        let connection = database::connection().await;
        Self::sync(connection).await.map_err(Error::Wither)
    }
}
