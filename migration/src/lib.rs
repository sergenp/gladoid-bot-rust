pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_gladiator;
mod m20221008_195747_create_table_attack_type;
mod m20221008_200725_create_table_attack_types_gladiators;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_gladiator::Migration),
            Box::new(m20221008_195747_create_table_attack_type::Migration),
            Box::new(m20221008_200725_create_table_attack_types_gladiators::Migration),
        ]
    }
}
