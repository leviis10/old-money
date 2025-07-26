use crate::m20250701_134445_create_m2m_user_roles_tables::Users;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BudgetConfigs::Table)
                    .if_not_exists()
                    .col(pk_auto(BudgetConfigs::Id))
                    .col(integer(BudgetConfigs::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-budget-configs_user-id")
                            .from(BudgetConfigs::Table, BudgetConfigs::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(string(BudgetConfigs::Name))
                    .col(integer(BudgetConfigs::Duration))
                    .col(decimal(BudgetConfigs::Limit))
                    .col(text_null(BudgetConfigs::Description))
                    .col(date(BudgetConfigs::LastCreate))
                    .col(
                        timestamp_with_time_zone(BudgetConfigs::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(BudgetConfigs::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(BudgetConfigs::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BudgetConfigs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BudgetConfigs {
    Table,
    Id,
    UserId,
    Name,
    Duration,
    Limit,
    Description,
    LastCreate,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
