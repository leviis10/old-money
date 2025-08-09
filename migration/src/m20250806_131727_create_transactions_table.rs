use crate::extension::postgres::Type;
use crate::m20250701_134445_create_m2m_user_roles_tables::Users;
use crate::m20250712_000001_create_categories_table::Categories;
use crate::m20250728_130953_create_wallets_table::Wallets;
use crate::m20250731_120654_create_budgets_table::Budgets;
use crate::sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create TransactionType enum
        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionType)
                    .values(TransactionTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        // create Transactions table
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(pk_auto(Transactions::Id))
                    .col(integer(Transactions::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_users_user-id")
                            .from(Transactions::Table, Transactions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Transactions::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_categories_category-id")
                            .from(Transactions::Table, Transactions::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(Transactions::BudgetId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_budgets_budget-id")
                            .from(Transactions::Table, Transactions::BudgetId)
                            .to(Budgets::Table, Budgets::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Transactions::WalletId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_wallets_wallet-id")
                            .from(Transactions::Table, Transactions::WalletId)
                            .to(Wallets::Table, Wallets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(decimal(Transactions::Amount))
                    .col(string_null(Transactions::Description))
                    .col(enumeration(
                        Transactions::FlowDirection,
                        TransactionType,
                        TransactionTypeVariants::iter(),
                    ))
                    .col(date(Transactions::IssuedAt).default(Expr::current_date()))
                    .col(
                        timestamp_with_time_zone(Transactions::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Transactions::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Transactions::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // drop the table
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await?;

        // drop enum type
        manager
            .drop_type(Type::drop().name(TransactionType).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
struct TransactionType;

#[derive(DeriveIden, EnumIter)]
enum TransactionTypeVariants {
    #[sea_orm(iden = "INCOME")]
    Income,

    #[sea_orm(iden = "OUTCOME")]
    Outcome,
}

#[derive(DeriveIden)]
enum Transactions {
    Table,
    Id,
    UserId,
    CategoryId,
    BudgetId,
    WalletId,
    Amount,
    Description,
    FlowDirection,
    IssuedAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
