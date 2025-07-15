use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::get_all_categories_params::GetAllCategoriesParams;
use crate::entities::{categories, users};
use crate::errors::AppError;
use crate::repositories::categories_repository;
use sea_orm::{ActiveValue, DatabaseConnection};

pub async fn create(
    db: &DatabaseConnection,
    user: &users::Model,
    request: &CreateCategoryRequest,
) -> Result<categories::Model, AppError> {
    let new_category = categories::ActiveModel {
        name: ActiveValue::Set(String::from(&request.name)),
        user_id: ActiveValue::Set(user.id),
        ..Default::default()
    };

    let category_model = categories_repository::create(db, new_category).await?;

    Ok(category_model)
}

pub async fn get_by_user_id_and_name(
    db: &DatabaseConnection,
    user_id: i32,
    category_name: &str,
) -> Result<categories::Model, AppError> {
    let Some(found_category) =
        categories_repository::get_by_user_id_and_name_ilike(db, user_id, category_name).await?
    else {
        return Err(AppError::NotFoundError(String::from("Category not found")));
    };

    Ok(found_category)
}

pub async fn get_all(
    db: &DatabaseConnection,
    user_id: i32,
    params: &GetAllCategoriesParams,
) -> Result<(Vec<categories::Model>, u64), AppError> {
    let (found_categories_paginated, total_found_categories) =
        categories_repository::get_all_by_user_id(db, user_id, params.page, params.page_size)
            .await?;
    Ok((found_categories_paginated, total_found_categories))
}
