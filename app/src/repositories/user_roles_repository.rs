use crate::entities::user_roles;
use crate::errors::AppError;
use sea_orm::{ActiveModelTrait, DatabaseTransaction};

pub async fn create_manual(
    txn: &DatabaseTransaction,
    new_user_role: user_roles::ActiveModel,
) -> Result<user_roles::Model, AppError> {
    let user_role_model = new_user_role.insert(txn).await?;
    Ok(user_role_model)
}
