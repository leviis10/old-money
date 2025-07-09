use crate::dto::request::roles_dto::find_role_by_name_request::FindRoleByNameRequest;
use crate::entities::roles;
use crate::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

pub async fn find_by_name(
    txn: &DatabaseTransaction,
    request: FindRoleByNameRequest,
) -> Result<roles::Model, AppError> {
    let found_role_option = roles::Entity::find()
        .filter(roles::Column::Name.eq(request.role.get_name()))
        .one(txn)
        .await?;

    match found_role_option {
        Some(found_role) => Ok(found_role),
        None => Err(AppError::NotFound(String::from("Role Not Found"))),
    }
}
