use crate::extension::postgres::Type;
use crate::m20250701_134445_create_m2m_user_roles_tables::Users;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(RepetitionTypeEnum)
                    .values(RepetitionTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        // create table
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
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(text_null(BudgetConfigs::Description))
                    .col(decimal(BudgetConfigs::Limit))
                    .col(string(BudgetConfigs::Name))
                    .col(enumeration(
                        BudgetConfigs::RepetitionType,
                        RepetitionTypeEnum,
                        RepetitionTypeVariants::iter(),
                    ))
                    .col(date_null(BudgetConfigs::LastCreate))
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
        // drop table
        manager
            .drop_table(Table::drop().table(BudgetConfigs::Table).to_owned())
            .await?;

        // drop enum type
        manager
            .drop_type(Type::drop().name(RepetitionTypeEnum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
struct RepetitionTypeEnum;

#[derive(DeriveIden, EnumIter)]
enum RepetitionTypeVariants {
    #[sea_orm(iden = "DAILY")]
    Daily,

    #[sea_orm(iden = "WEEKLY")]
    Weekly,

    #[sea_orm(iden = "MONTHLY")]
    Monthly,

    #[sea_orm(iden = "YEARLY")]
    Yearly,
}

#[derive(DeriveIden)]
pub enum BudgetConfigs {
    Table,
    Id,
    UserId,
    Description,
    Limit,
    Name,
    RepetitionType,
    LastCreate,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
