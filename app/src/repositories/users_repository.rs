use crate::entities::prelude::Users;
use crate::entities::{roles, users};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DatabaseTransaction,
    EntityTrait, QueryFilter,
};

pub async fn create_manual(
    txn: &DatabaseTransaction,
    new_user_username: &str,
    new_user_email: &str,
    new_user_hashed_password: &str,
) -> users::Model {
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(String::from(new_user_username)),
        email: ActiveValue::Set(String::from(new_user_email)),
        password: ActiveValue::Set(String::from(new_user_hashed_password)),
        ..Default::default()
    };

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

pub async fn find_by_pk(
    db: &DatabaseConnection,
    user_id: i32,
) -> (users::Model, Vec<roles::Model>) {
    let found_users = Users::find()
        .filter(users::Column::Id.eq(user_id))
        .find_with_related(roles::Entity)
        .all(db)
        .await
        .unwrap();

    found_users.get(0).unwrap().clone()
}
