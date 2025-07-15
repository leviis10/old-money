use crate::entities::categories;
use crate::entities::prelude::Categories;
use crate::errors::AppError;
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

pub async fn create(
    db: &DatabaseConnection,
    entity: categories::ActiveModel,
) -> Result<categories::Model, AppError> {
    let model = entity.insert(db).await?;
    Ok(model)
}

pub async fn get_by_user_id_and_name_ilike(
    db: &DatabaseConnection,
    user_id: i32,
    category_name: &str,
) -> Result<Option<categories::Model>, AppError> {
    let found_category = Categories::find()
        .filter(categories::Column::UserId.eq(user_id))
        .filter(Expr::col(categories::Column::Name).ilike(category_name))
        .one(db)
        .await?;
    Ok(found_category)
}

pub async fn get_all_by_user_id(
    db: &DatabaseConnection,
    user_id: i32,
    page: Option<u64>,
    page_size: Option<u64>,
) -> Result<(Vec<categories::Model>, u64), AppError> {
    let mut found_categories_builder = Categories::find()
        .filter(categories::Column::UserId.eq(user_id))
        .order_by_desc(categories::Column::CreatedAt);
    if let Some(page) = page {
        found_categories_builder = found_categories_builder.offset((page - 1) * page_size.unwrap());
    }
    if let Some(per_page) = page_size {
        found_categories_builder = found_categories_builder.limit(per_page);
    }
    let found_categories = found_categories_builder.all(db).await?;

    let total_categories_found = Categories::find()
        .filter(categories::Column::UserId.eq(user_id))
        .count(db)
        .await?;
    Ok((found_categories, total_categories_found))
}
