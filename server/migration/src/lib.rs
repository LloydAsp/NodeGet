pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20260113_044428_create_static_monitoring::Migration,
        )]
    }
}
mod m20260113_044428_create_static_monitoring;
