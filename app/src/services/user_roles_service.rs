use crate::entities::user_roles;
use sea_orm::{ActiveModelTrait, DatabaseTransaction};

pub async fn insert_manual(
    txn: &DatabaseTransaction,
    new_user_role: user_roles::ActiveModel,
) -> user_roles::Model {
    new_user_role.insert(txn).await.unwrap()
}
