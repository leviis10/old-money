use crate::dto::request::categories_dto::get_all_categories_params::ValidatedGetAllCategoriesParams;
use crate::dto::request::categories_dto::update_category_request::UpdateCategoryRequest;
use crate::entities::categories;
use crate::entities::prelude::Categories;
use crate::errors::AppError;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    ItemsAndPagesNumber, PaginatorTrait, QueryFilter, QueryOrder,
};
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    entity: categories::ActiveModel,
) -> Result<categories::Model, AppError> {
    let model = entity.insert(db).await?;
    Ok(model)
}

pub async fn get_all_by_user_id(
    db: &DatabaseConnection,
    user_id: i32,
    params: ValidatedGetAllCategoriesParams,
) -> Result<(Vec<categories::Model>, Option<ItemsAndPagesNumber>), AppError> {
    let mut found_categories_builder = Categories::find()
        .filter(categories::Column::UserId.eq(user_id))
        .filter(categories::Column::DeletedAt.is_null())
        .order_by_asc(categories::Column::Name);

    if let Some(name) = params.name {
        found_categories_builder = found_categories_builder
            .filter(Expr::col(categories::Column::Name).ilike(format!("%{name}%")));
    }

    let Some(paginated) = params.paginated else {
        let found_categories = found_categories_builder.all(db).await?;
        return Ok((found_categories, None));
    };

    let paginator = found_categories_builder.paginate(db, paginated.page_size);
    let found_categories = paginator.fetch_page(paginated.page - 1).await?;

    let page_information = paginator.num_items_and_pages().await?;

    Ok((found_categories, Some(page_information)))
}

pub async fn update_by_user_id_and_id(
    db: &DatabaseConnection,
    user_id: i32,
    category_id: i32,
    request: &UpdateCategoryRequest,
) -> Result<categories::Model, AppError> {
    let mut found_category_model = find_by_user_id_and_id(db, user_id, category_id)
        .await?
        .into_active_model();
    found_category_model.name = ActiveValue::Set(String::from(&request.name));
    found_category_model.updated_at = ActiveValue::Set(OffsetDateTime::now_utc());

    let result = found_category_model.update(db).await?;
    Ok(result)
}

pub async fn find_by_user_id_and_id(
    db: &DatabaseConnection,
    user_id: i32,
    id: i32,
) -> Result<categories::Model, AppError> {
    let Some(found_category_model) = Categories::find()
        .filter(categories::Column::Id.eq(id))
        .filter(categories::Column::UserId.eq(user_id))
        .one(db)
        .await?
    else {
        return Err(AppError::NotFound(String::from("Category not found")));
    };

    Ok(found_category_model)
}

pub async fn delete_by_user_id_and_id(
    db: &DatabaseConnection,
    user_id: i32,
    category_id: i32,
) -> Result<(), AppError> {
    let mut found_category = find_by_user_id_and_id(db, user_id, category_id)
        .await?
        .into_active_model();
    found_category.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));
    found_category.update(db).await?;
    Ok(())
}
