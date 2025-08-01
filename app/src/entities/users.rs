//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
    pub deleted_at: Option<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::budget_configs::Entity")]
    BudgetConfigs,
    #[sea_orm(has_many = "super::budgets::Entity")]
    Budgets,
    #[sea_orm(has_many = "super::categories::Entity")]
    Categories,
    #[sea_orm(has_many = "super::refresh_tokens::Entity")]
    RefreshTokens,
    #[sea_orm(has_many = "super::user_roles::Entity")]
    UserRoles,
    #[sea_orm(has_many = "super::wallets::Entity")]
    Wallets,
}

impl Related<super::budget_configs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetConfigs.def()
    }
}

impl Related<super::budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Budgets.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl Related<super::user_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::wallets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wallets.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_roles::Relation::Roles.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_roles::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
