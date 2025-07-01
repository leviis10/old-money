pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250627_000001_create_categories_table;
mod m20250701_134445_create_m2m_user_roles_tables;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250627_000001_create_categories_table::Migration),
            Box::new(m20250701_134445_create_m2m_user_roles_tables::Migration),
        ]
    }
}
