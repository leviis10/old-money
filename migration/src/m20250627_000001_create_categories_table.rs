use sea_orm_migration::prelude::*;

pub struct Migration;

#[derive(Iden)]
pub enum Categories {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250627_000001_create_categories_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .col(
                        ColumnDef::new(Categories::Id)
                            .primary_key()
                            .big_integer()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Categories::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}
