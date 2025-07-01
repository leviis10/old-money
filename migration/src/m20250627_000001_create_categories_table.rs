use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_auto, string, timestamp_with_time_zone, timestamp_with_time_zone_null};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(pk_auto(Categories::Id))
                    .col(string(Categories::Name))
                    .col(timestamp_with_time_zone(Categories::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Categories::UpdatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone_null(Categories::DeletedAt))
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Categories {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
