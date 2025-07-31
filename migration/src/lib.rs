pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250701_134445_create_m2m_user_roles_tables;
mod m20250707_132143_create_refresh_tokens_table;
mod m20250712_000001_create_categories_table;
mod m20250726_160856_create_budget_configs_table;
mod m20250728_130953_create_wallets_table;
mod m20250731_120654_create_budgets_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250701_134445_create_m2m_user_roles_tables::Migration),
            Box::new(m20250707_132143_create_refresh_tokens_table::Migration),
            Box::new(m20250712_000001_create_categories_table::Migration),
            Box::new(m20250726_160856_create_budget_configs_table::Migration),
            Box::new(m20250728_130953_create_wallets_table::Migration),
            Box::new(m20250731_120654_create_budgets_table::Migration),
        ]
    }
}
