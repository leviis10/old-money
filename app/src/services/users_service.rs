use crate::dto::request::roles_dto::find_role_by_name_request::FindRoleByNameRequest;
use crate::dto::request::user_roles_dto::insert_user_role_manual_request::InsertUserRoleManualRequest;
use crate::dto::request::users_dto::create_user_request::CreateUserRequest;
use crate::dto::request::users_dto::find_user_by_pk_request::FindUserByPkRequest;
use crate::dto::request::users_dto::find_user_by_username_request::FindUserByUsernameRequest;
use crate::entities::{roles, users};
use crate::repositories::users_repository;
use crate::services::{roles_service, user_roles_service};
use sea_orm::{ActiveValue, DatabaseConnection, TransactionTrait};

pub async fn create(db: &DatabaseConnection, request: CreateUserRequest) -> users::Model {
    let txn = db.begin().await.unwrap();
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(request.username),
        email: ActiveValue::Set(request.email),
        password: ActiveValue::Set(request.hashed_password),
        ..Default::default()
    };
    let user_model = users_repository::create_manual(&txn, new_user).await;

    for role in request.roles.into_iter() {
        let role_model = roles_service::find_by_name(&txn, FindRoleByNameRequest { role }).await;
        user_roles_service::create_manual(
            &txn,
            InsertUserRoleManualRequest {
                user_id: user_model.id,
                role_id: role_model.id,
            },
        )
        .await;
    }

    txn.commit().await.unwrap();

    user_model
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    request: FindUserByUsernameRequest,
) -> (users::Model, Vec<roles::Model>) {
    users_repository::find_by_username(db, &request.username).await
}

pub async fn find_by_pk(
    db: &DatabaseConnection,
    request: FindUserByPkRequest,
) -> (users::Model, Vec<roles::Model>) {
    users_repository::find_by_pk(db, request.user_id).await
}
