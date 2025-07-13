use crate::AppState;
use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::dto::response::categories_dto::create_category_response::CreateCategoryResponse;
use crate::dto::response::categories_dto::get_category_response::GetCategoryResponse;
use crate::dto::response::categories_dto::update_category_response::UpdateCategoryResponse;
use crate::dto::response::global::success_response::{Meta, SuccessResponse};
use crate::entities::categories;
use crate::entities::prelude::Categories;
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::user::User;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use sea_orm::{ActiveValue, EntityTrait};
use std::sync::Arc;
use time::OffsetDateTime;

pub async fn get_all() -> Result<SuccessResponse<Vec<GetCategoryResponse>>, AppError> {
    let response = vec![
        GetCategoryResponse {
            id: 1u32,
            name: String::from("category 1"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        },
        GetCategoryResponse {
            id: 2u32,
            name: String::from("category 2"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        },
    ];
    let meta = Meta::new(response.len() as u64, 1, 1);

    Ok(SuccessResponse::new("Successfully get all categories", response).with_meta(meta))
}

pub async fn get_by_id(
    Path(id): Path<u32>,
) -> Result<SuccessResponse<GetCategoryResponse>, AppError> {
    let response = GetCategoryResponse {
        id,
        name: format!("category {id}"),
        created_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };
    Ok(SuccessResponse::new("Successfully get category", response))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateCategoryResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::Admin, Roles::User])?;

    let new_category = categories::ActiveModel {
        name: ActiveValue::Set(String::from(&payload.name)),
        user_id: ActiveValue::Set(found_user.id),
        ..Default::default()
    };
    let db_response = Categories::insert(new_category).exec(&state.db).await?;

    let now = OffsetDateTime::now_utc();
    let response = CreateCategoryResponse {
        id: db_response.last_insert_id,
        name: payload.name,
        created_at: now,
        updated_at: now,
    };

    Ok((
        StatusCode::CREATED,
        SuccessResponse::new("Successfully created new category", response),
    ))
}

pub async fn update_by_id(
    Path(id): Path<u32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> SuccessResponse<UpdateCategoryResponse> {
    let response = UpdateCategoryResponse {
        id,
        name: payload.name,
        created_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };
    SuccessResponse::new("Successfully updated category", response)
}

pub async fn delete_by_id(Path(id): Path<u32>) -> StatusCode {
    println!("Deleting category with id of {id}");
    StatusCode::NO_CONTENT
}
