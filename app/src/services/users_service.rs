use crate::dto::request::roles_dto::find_role_by_name_request::FindRoleByNameRequest;
use crate::dto::request::user_roles_dto::insert_user_role_manual_request::InsertUserRoleManualRequest;
use crate::dto::request::users_dto::create_user_request::CreateUserRequest;
use crate::dto::request::users_dto::delete_self_request::DeleteSelfRequest;
use crate::dto::request::users_dto::find_user_by_pk_request::FindUserByPkRequest;
use crate::dto::request::users_dto::find_user_by_username_request::FindUserByUsernameRequest;
use crate::dto::request::users_dto::update_self_request::UpdateSelfRequest;
use crate::entities::{roles, users};
use crate::errors::AppError;
use crate::repositories::users_repository;
use crate::services::{roles_service, user_roles_service};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, IntoActiveModel, TransactionTrait,
    TryIntoModel,
};
use time::OffsetDateTime;

pub async fn create(
    db: &DatabaseConnection,
    request: CreateUserRequest,
) -> Result<users::Model, AppError> {
    let txn = db.begin().await?;
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(request.username),
        email: ActiveValue::Set(request.email),
        password: ActiveValue::Set(request.hashed_password),
        ..Default::default()
    };
    let user_model = users_repository::create_manual(&txn, new_user).await?;

    for role in request.roles.into_iter() {
        let role_model = roles_service::find_by_name(&txn, FindRoleByNameRequest { role }).await?;
        user_roles_service::create_manual(
            &txn,
            InsertUserRoleManualRequest {
                user_id: user_model.id,
                role_id: role_model.id,
            },
        )
        .await?;
    }

    txn.commit().await?;

    Ok(user_model)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    request: FindUserByUsernameRequest,
) -> Result<(users::Model, Vec<roles::Model>), AppError> {
    let found_user_option = users_repository::find_by_username(db, &request.username).await?;
    match found_user_option {
        Some(found_user) => Ok(found_user),
        None => Err(AppError::NotFound(String::from("User Not Found"))),
    }
}

pub async fn find_by_pk(
    db: &DatabaseConnection,
    request: FindUserByPkRequest,
) -> Result<(users::Model, Vec<roles::Model>), AppError> {
    let found_user_option = users_repository::find_by_pk(db, request.user_id).await?;
    match found_user_option {
        Some(found_user) => Ok(found_user),
        None => Err(AppError::NotFound(String::from("User Not Found"))),
    }
}

pub async fn update_using_model(
    db: &DatabaseConnection,
    user_model: users::Model,
    payload: &UpdateSelfRequest,
) -> Result<users::Model, AppError> {
    let parsed_hash = PasswordHash::new(&user_model.password)?;
    Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash)?;

    let mut user_model = user_model.into_active_model();
    user_model.username = ActiveValue::Set(String::from(&payload.username));
    user_model.email = ActiveValue::Set(String::from(&payload.email));
    let updated_user = user_model.save(db).await?.try_into_model()?;

    Ok(updated_user)
}

pub async fn delete_using_model(
    db: &DatabaseConnection,
    user_model: users::Model,
    payload: &DeleteSelfRequest,
) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(&user_model.password)?;
    Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash)?;

    let mut user_model = user_model.into_active_model();
    user_model.deleted_at = ActiveValue::Set(Some(OffsetDateTime::now_utc()));
    user_model.save(db).await?;

    Ok(())
}
