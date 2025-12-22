use axum::http::StatusCode;
use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};
use bson::doc;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wither::mongodb::options::FindOptions;

use crate::errors::Error;
use crate::models::cat::{Cat, PublicCat};
use crate::utils::custom_response::CustomResponseResult as Response;
use crate::utils::custom_response::{CustomResponse, CustomResponseBuilder, ResponsePagination};
use crate::utils::models::ModelExt;
use crate::utils::pagination::Pagination;
use crate::utils::to_object_id::to_object_id;
use crate::utils::token::TokenUser;

/*
Enterprise notes (for devs):
- This module always returns the same API envelope via CustomResponseBuilder.
- Avoid `unwrap()` in request paths—turn failures into structured API errors.
- Validate user input (basic sanity) before hitting the DB.
- If you want stricter validation, add validator crate and enforce min/max lengths.
*/

pub fn create_route() -> Router {
    Router::new()
        .route("/cats", post(create_cat).get(query_cats))
        .route(
            "/cats/:id",
            get(get_cat_by_id).delete(remove_cat_by_id).put(update_cat_by_id),
        )
}

async fn create_cat(user: TokenUser, Json(payload): Json<CreateCat>) -> Response<PublicCat> {
    // Basic validation (keep it cheap)
    let name = payload.name.trim();
    if name.is_empty() {
        // If your Error has a better constructor (bad_request/validation), use it here.
        return Err(Error::bad_request("name is required"));
    }

    let cat = Cat::new(user.id, name.to_string());
    let cat = Cat::create(cat).await?;
    let body = PublicCat::from(cat);

    let res = CustomResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .body(body)
        .build();

    Ok(res)
}

async fn query_cats(user: TokenUser, pagination: Pagination) -> Response<Vec<PublicCat>> {
    // Defensive pagination (avoid accidental "give me the whole DB")
    let limit = pagination.limit.min(100).max(1);
    let offset = pagination.offset.max(0);

    let options = FindOptions::builder()
        .sort(doc! { "created_at": -1_i32 })
        .skip(offset)
        .limit(limit as i64)
        .build();

    let (cats, count) = Cat::find_and_count(doc! { "user": &user.id }, options).await?;
    let cats = cats.into_iter().map(PublicCat::from).collect::<Vec<_>>();

    let res = CustomResponseBuilder::new()
        .body(cats)
        .pagination(ResponsePagination {
            count,
            offset,
            limit,
        })
        .build();

    debug!("Returning cats (count={count}, offset={offset}, limit={limit})");
    Ok(res)
}

async fn get_cat_by_id(user: TokenUser, Path(id): Path<String>) -> Response<PublicCat> {
    let cat_id = to_object_id(id)?;
    let cat = Cat::find_one(doc! { "_id": cat_id, "user": &user.id }, None)
        .await?
        .map(PublicCat::from)
        .ok_or_else(Error::not_found)?;

    let res = CustomResponseBuilder::new().body(cat).build();

    debug!("Returning cat");
    Ok(res)
}

async fn remove_cat_by_id(user: TokenUser, Path(id): Path<String>) -> Result<CustomResponse<()>, Error> {
    let cat_id = to_object_id(id)?;
    let delete_result = Cat::delete_one(doc! { "_id": cat_id, "user": &user.id }).await?;

    if delete_result.deleted_count == 0 {
        debug!("Cat not found, returning 404 status code");
        return Err(Error::not_found());
    }

    let res = CustomResponseBuilder::new()
        .status_code(StatusCode::NO_CONTENT)
        .build();

    Ok(res)
}

async fn update_cat_by_id(
    user: TokenUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCat>,
) -> Response<PublicCat> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(Error::bad_request("name is required"));
    }

    let cat_id = to_object_id(id)?;

    // Avoid unwrap: bson conversion can fail (rare, but don’t explode)
    let update_doc = bson::to_document(&UpdateCat {
        name: name.to_string(),
    })
    .map_err(|_| Error::bad_request("invalid payload"))?;

    // NOTE for devs:
    // - If your `find_one_and_update` returns the *old* doc by default,
    //   switch to options(ReturnDocument::After) in your ModelExt helper.
    let cat = Cat::find_one_and_update(
        doc! { "_id": &cat_id, "user": &user.id },
        doc! { "$set": update_doc },
    )
    .await?
    .map(PublicCat::from)
    .ok_or_else(Error::not_found)?;

    let res = CustomResponseBuilder::new().body(cat).build();

    debug!("Returning updated cat");
    Ok(res)
}

#[derive(Deserialize)]
struct CreateCat {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateCat {
    name: String,
}
