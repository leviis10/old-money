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
                    .table(RefreshTokens::Table)
                    .if_not_exists()
                    .col(pk_uuid(RefreshTokens::Jti))
                    .col(string(RefreshTokens::HashedToken))
                    .col(timestamp_with_time_zone(RefreshTokens::ExpiresAt))
                    .col(
                        timestamp_with_time_zone(RefreshTokens::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(RefreshTokens::DeletedAt))
                    .col(integer(RefreshTokens::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refresh_tokens-user-id")
                            .from(RefreshTokens::Table, RefreshTokens::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RefreshTokens {
    Table,
    Jti,
    HashedToken,
    ExpiresAt,
    CreatedAt,
    DeletedAt,
    UserId,
}
