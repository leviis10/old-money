use crate::entities::prelude::Users;
use crate::entities::{roles, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter,
};

pub async fn create_manual(
    txn: &DatabaseTransaction,
    new_user: users::ActiveModel,
) -> users::Model {
    new_user.insert(txn).await.unwrap()
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> (users::Model, Vec<roles::Model>) {
    let found_users = Users::find()
        .filter(users::Column::Username.eq(username))
        .find_with_related(roles::Entity)
        .all(db)
        .await
        .unwrap();

    found_users.get(0).unwrap().clone()
}

pub async fn find_by_pk(db: &DatabaseConnection, user_id: i32) -> (users::Model, Vec<roles::Model>) {
    let found_users = Users::find()
        .filter(users::Column::Id.eq(user_id))
        .find_with_related(roles::Entity)
        .all(db)
        .await
        .unwrap();

    found_users.get(0).unwrap().clone()
}
