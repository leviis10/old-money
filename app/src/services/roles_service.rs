use crate::dto::request::roles_dto::find_role_by_name_request::FindRoleByNameRequest;
use crate::entities::roles;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

pub async fn find_by_name(
    txn: &DatabaseTransaction,
    request: FindRoleByNameRequest,
) -> roles::Model {
    roles::Entity::find()
        .filter(roles::Column::Name.eq(request.role.get_name()))
        .one(txn)
        .await
        .unwrap()
        .unwrap()
}
