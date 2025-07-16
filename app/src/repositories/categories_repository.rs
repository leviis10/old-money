use crate::dto::request::categories_dto::get_all_categories_params::ValidatedGetAllCategoriesParams;
use crate::entities::categories;
use crate::entities::prelude::Categories;
use crate::errors::AppError;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ItemsAndPagesNumber,
    PaginatorTrait, QueryFilter, QueryOrder,
};

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
        .order_by_desc(categories::Column::CreatedAt);

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
