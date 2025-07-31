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
                    .table(Wallets::Table)
                    .if_not_exists()
                    .col(pk_auto(Wallets::Id))
                    .col(integer(Wallets::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallets_user-id")
                            .from(Wallets::Table, Wallets::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Wallets::Name))
                    .col(decimal(Wallets::Balance).default(0))
                    .col(text_null(Wallets::Description))
                    .col(
                        timestamp_with_time_zone(Wallets::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Wallets::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Wallets::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wallets {
    Table,
    Id,
    UserId,
    Name,
    Balance,
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
