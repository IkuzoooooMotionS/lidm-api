pub use sea_orm_migration::prelude::*;

mod m20250223_222316_create_users_table;
mod m20250223_222336_create_user_roles_table;
mod m20250223_222341_create_roles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250223_222316_create_users_table::Migration),
            Box::new(m20250223_222336_create_user_roles_table::Migration),
            Box::new(m20250223_222341_create_roles_table::Migration),
        ]
    }
}
