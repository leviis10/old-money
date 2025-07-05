use crate::entities::roles;
use crate::enums::roles::Roles;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

pub async fn find_by_name(txn: &DatabaseTransaction, role: &Roles) -> roles::Model {
    roles::Entity::find()
        .filter(roles::Column::Name.eq(role.get_name()))
        .one(txn)
        .await
        .unwrap()
        .unwrap()
}
