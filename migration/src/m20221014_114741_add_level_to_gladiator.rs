use sea_orm_migration::prelude::*;


use super::m20220101_000001_create_table_gladiator::Gladiator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Gladiator::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("level"))
                            .integer()
                            .not_null()
                            .default(1)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // no down implementation for this migration 
        // because sqlite doesn't support dropping columns
        return Ok(());
    }
}

