use crate::m20250701_134445_create_m2m_user_roles_tables::Users;
use crate::m20250726_160856_create_budget_configs_table::BudgetConfigs;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Budgets::Table)
                    .if_not_exists()
                    .col(pk_auto(Budgets::Id))
                    .col(integer(Budgets::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_budgets_users_user-id")
                            .from(Budgets::Table, Budgets::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Budgets::BudgetConfigId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_budgets_budget-configs_budget-config-id")
                            .from(Budgets::Table, Budgets::BudgetConfigId)
                            .to(BudgetConfigs::Table, BudgetConfigs::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Budgets::Name))
                    .col(date(Budgets::StartDate))
                    .col(date(Budgets::EndDate))
                    .col(decimal(Budgets::CurrentAmount).default(0))
                    .col(decimal(Budgets::Limit))
                    .col(text_null(Budgets::Description))
                    .col(
                        timestamp_with_time_zone(Budgets::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Budgets::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Budgets::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Budgets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Budgets {
    Table,
    Id,
    UserId,
    BudgetConfigId,
    Name,
    StartDate,
    EndDate,
    CurrentAmount,
    Limit,
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
