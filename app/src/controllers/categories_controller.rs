use crate::AppState;
use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::get_all_categories_params::GetAllCategoriesParams;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::dto::response::categories_dto::create_category_response::CreateCategoryResponse;
use crate::dto::response::categories_dto::get_category_response::GetCategoryResponse;
use crate::dto::response::categories_dto::update_category_response::UpdateCategoryResponse;
use crate::dto::response::global::success_response::{Meta, SuccessResponse};
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::extractors::json::ValidatedJson;
use crate::extractors::user::User;
use crate::services::categories_service;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use std::sync::Arc;
use time::OffsetDateTime;

#[utoipa::path(
    tag = "categories",
    get,
    path = "/api/v1/categories",
    params(
        ("page" = Option<String>, Query),
        ("page_size" = Option<String>, Query),
        ("name" = Option<String>, Query),
    ),
    responses(
        (status = 200, body = SuccessResponse<Vec<GetCategoryResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_all(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    Query(params): Query<GetAllCategoriesParams>,
) -> Result<(StatusCode, SuccessResponse<Vec<GetCategoryResponse>>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    let validated_query_params = params.validate()?;

    let (found_categories, total_found_categories) =
        categories_service::get_all(&state.db, found_user.id, validated_query_params.to_owned())
            .await?;

    let found_categories = found_categories
        .iter()
        .map(|category| GetCategoryResponse {
            id: category.id,
            name: String::from(&category.name),
            created_at: category.created_at,
            updated_at: category.updated_at,
        })
        .collect();

    let (Some(paginated), Some(page_information)) =
        (validated_query_params.paginated, total_found_categories)
    else {
        return Ok((
            StatusCode::OK,
            SuccessResponse::new("Successfully get all categories", found_categories),
        ));
    };

    let meta = Meta {
        total_items: page_information.number_of_items,
        page: paginated.page,
        page_size: paginated.page_size,
        last_page: page_information.number_of_pages,
    };

    Ok((
        StatusCode::OK,
        SuccessResponse::new("Successfully get all categories", found_categories).with_meta(meta),
    ))
}

#[utoipa::path(
    tag = "categories",
    post,
    path = "/api/v1/categories",
    request_body(
        content = CreateCategoryRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, body = SuccessResponse<CreateCategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    State(state): State<Arc<AppState>>,
    User(found_user, roles): User,
    ValidatedJson(payload): ValidatedJson<CreateCategoryRequest>,
) -> Result<(StatusCode, SuccessResponse<CreateCategoryResponse>), AppError> {
    User::has_any_role(roles, vec![Roles::User])?;
    let new_category_model = categories_service::create(&state.db, &found_user, &payload).await?;
    Ok((
        StatusCode::CREATED,
        SuccessResponse::new(
            "Successfully created new category",
            CreateCategoryResponse {
                id: new_category_model.id,
                name: new_category_model.name,
                created_at: new_category_model.created_at,
                updated_at: new_category_model.updated_at,
            },
        ),
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
