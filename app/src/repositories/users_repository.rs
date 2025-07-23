use crate::entities::prelude::Users;
use crate::entities::{roles, users};
use crate::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter,
};

pub async fn create_manual(
    txn: &DatabaseTransaction,
    new_user: users::ActiveModel,
) -> Result<users::Model, AppError> {
    let new_user_model = new_user.insert(txn).await?;
    Ok(new_user_model)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<(users::Model, Vec<roles::Model>)>, AppError> {
    let found_users = Users::find()
        .filter(users::Column::Username.eq(username))
        .filter(users::Column::DeletedAt.is_null())
        .find_with_related(roles::Entity)
        .all(db)
        .await?;

    Ok(get_first_user(found_users))
}

pub async fn find_by_pk(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<(users::Model, Vec<roles::Model>)>, AppError> {
    let found_users = Users::find()
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::DeletedAt.is_null())
        .find_with_related(roles::Entity)
        .all(db)
        .await?;

    Ok(get_first_user(found_users))
}

fn get_first_user(
    found_users: Vec<(users::Model, Vec<roles::Model>)>,
) -> Option<(users::Model, Vec<roles::Model>)> {
    let (found_user, roles) = found_users.first()?;
    Some((found_user.to_owned(), roles.to_owned()))
}
