use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::get_all_categories_params::ValidatedGetAllCategoriesParams;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::entities::{categories, users};
use crate::errors::AppError;
use crate::repositories::categories_repository;
use sea_orm::{ActiveValue, ConnectionTrait, DatabaseConnection, ItemsAndPagesNumber};

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

pub async fn find_all(
    db: &DatabaseConnection,
    user_id: i32,
    params: ValidatedGetAllCategoriesParams,
) -> Result<(Vec<categories::Model>, Option<ItemsAndPagesNumber>), AppError> {
    let (found_categories, page_information) =
        categories_repository::get_all_by_user_id(db, user_id, params).await?;
    Ok((found_categories, page_information))
}

pub async fn get_by_id(
    connection: &impl ConnectionTrait,
    user: &users::Model,
    category_id: i32,
) -> Result<categories::Model, AppError> {
    let found_category =
        categories_repository::get_active_by_id_and_user_id(connection, category_id, user.id)
            .await?;
    let Some(found_category) = found_category else {
        return Err(AppError::NotFound(String::from("Category not found")));
    };
    Ok(found_category)
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    found_user: &users::Model,
    category_id: i32,
    request: &UpdateCategoryRequest,
) -> Result<categories::Model, AppError> {
    let updated_category_model =
        categories_repository::update_by_user_id_and_id(db, found_user.id, category_id, request)
            .await?;
    Ok(updated_category_model)
}

pub async fn delete_by_id(
    db: &DatabaseConnection,
    found_user: &users::Model,
    category_id: i32,
) -> Result<(), AppError> {
    categories_repository::delete_by_user_id_and_id(db, found_user.id, category_id).await?;
    Ok(())
}
