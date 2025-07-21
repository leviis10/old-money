use crate::dto::request::categories_dto::create_category_request::CreateCategoryRequest;
use crate::dto::request::categories_dto::get_all_categories_params::ValidatedGetAllCategoriesParams;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::entities::{categories, users};
use crate::errors::AppError;
use crate::repositories::categories_repository;
use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveModel, ItemsAndPagesNumber};

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

pub async fn get_all(
    db: &DatabaseConnection,
    user_id: i32,
    params: ValidatedGetAllCategoriesParams,
) -> Result<(Vec<categories::Model>, Option<ItemsAndPagesNumber>), AppError> {
    let (found_categories, page_information) =
        categories_repository::get_all_by_user_id(db, user_id, params).await?;
    Ok((found_categories, page_information))
}

pub async fn update_by_id(
    db: &DatabaseConnection,
    found_user: &users::Model,
    category_id: i32,
    request: &UpdateCategoryRequest,
) -> Result<categories::Model, AppError> {
    let mut found_category_model =
        categories_repository::find_by_user_id_and_id(db, found_user.id, category_id)
            .await?
            .into_active_model();
    found_category_model.name = ActiveValue::Set(String::from(&request.name));

    let updated_category_model =
        categories_repository::update_using_model(db, found_category_model).await?;
    Ok(updated_category_model)
}
