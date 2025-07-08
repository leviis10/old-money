use crate::entities::{roles, user_roles, users};
use crate::enums::roles::Roles;
use crate::repositories::users_repository;
use crate::services::{roles_service, user_roles_service};
use sea_orm::{ActiveValue, DatabaseConnection, TransactionTrait};

pub async fn create(
    database_connection: &DatabaseConnection,
    new_user_username: &str,
    new_user_email: &str,
    new_user_hashed_password: &str,
    roles: Vec<Roles>,
) -> users::Model {
    let txn = database_connection.begin().await.unwrap();
    let user_model = users_repository::create_manual(
        &txn,
        new_user_username,
        new_user_email,
        new_user_hashed_password,
    )
    .await;

    for role in roles.iter() {
        let role_model = roles_service::find_by_name(&txn, role).await;

        let new_user_role = user_roles::ActiveModel {
            user_id: ActiveValue::Set(user_model.id),
            role_id: ActiveValue::Set(role_model.id),
        };
        user_roles_service::insert_manual(&txn, new_user_role).await;
    }

    txn.commit().await.unwrap();

    user_model
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> (users::Model, Vec<roles::Model>) {
    users_repository::find_by_username(db, username).await
}

pub async fn find_by_pk(
    db: &DatabaseConnection,
    user_id: i32,
) -> (users::Model, Vec<roles::Model>) {
    users_repository::find_by_pk(db, user_id).await
}
