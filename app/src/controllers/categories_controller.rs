use crate::AppState;
use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::dto::response::categories_dto::create_category_response::{
    CreateCategoryResponse, CreateCategoryResponseBuilder,
};
use crate::dto::response::categories_dto::get_category_response::{
    GetCategoryResponse, GetCategoryResponseBuilder,
};
use crate::dto::response::categories_dto::update_category_response::{
    UpdateCategoryResponse, UpdateCategoryResponseBuilder,
};
use crate::dto::response::global::success_response::{Meta, SuccessResponse};
use crate::entities::categories;
use crate::entities::prelude::Categories;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use sea_orm::{ActiveValue, EntityTrait};
use std::sync::Arc;
use time::OffsetDateTime;

pub async fn get_all() -> SuccessResponse<Vec<GetCategoryResponse>> {
    let response = vec![
        GetCategoryResponseBuilder::default()
            .id(1u32)
            .name("category 1")
            .created_at(OffsetDateTime::now_utc())
            .updated_at(OffsetDateTime::now_utc())
            .build()
            .unwrap(),
        GetCategoryResponseBuilder::default()
            .id(2u32)
            .name("category 2")
            .created_at(OffsetDateTime::now_utc())
            .updated_at(OffsetDateTime::now_utc())
            .build()
            .unwrap(),
    ];
    let meta = Meta::new(response.len() as u64, 1, 1);

    SuccessResponse::new("Successfully get all categories", response).with_meta(meta)
}

pub async fn get_by_id(Path(id): Path<u32>) -> SuccessResponse<GetCategoryResponse> {
    let response = GetCategoryResponseBuilder::default()
        .id(id)
        .name(format!("category {id}"))
        .created_at(OffsetDateTime::now_utc())
        .updated_at(OffsetDateTime::now_utc())
        .build()
        .unwrap();
    SuccessResponse::new("Successfully get category", response)
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> (StatusCode, SuccessResponse<CreateCategoryResponse>) {
    let new_category = categories::ActiveModel {
        name: ActiveValue::Set(String::from(&payload.name)),
        ..Default::default()
    };
    let db_response = Categories::insert(new_category)
        .exec(&state.db)
        .await
        .unwrap();

    let now = OffsetDateTime::now_utc();
    let response = CreateCategoryResponseBuilder::default()
        .id(db_response.last_insert_id)
        .name(payload.name)
        .created_at(now)
        .updated_at(now)
        .build()
        .unwrap();

    (
        StatusCode::CREATED,
        SuccessResponse::new("Successfully created new category", response),
    )
}

pub async fn update_by_id(
    Path(id): Path<u32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> SuccessResponse<UpdateCategoryResponse> {
    let response = UpdateCategoryResponseBuilder::default()
        .id(id)
        .name(payload.name)
        .created_at(OffsetDateTime::now_utc())
        .updated_at(OffsetDateTime::now_utc())
        .build()
        .unwrap();
    SuccessResponse::new("Successfully updated category", response)
}

pub async fn delete_by_id(Path(id): Path<u32>) -> StatusCode {
    println!("Deleting category with id of {id}");
    StatusCode::NO_CONTENT
}
