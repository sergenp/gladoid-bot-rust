use super::m20220101_000001_create_table_gladiator::Gladiator;
use super::m20221008_195747_create_table_attack_type::AttackType;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttackTypesGladiators::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AttackTypesGladiators::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AttackTypesGladiators::AttackTypeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_attack_type_id")
                            .from(
                                AttackTypesGladiators::Table,
                                AttackTypesGladiators::AttackTypeId,
                            )
                            .to(AttackType::Table, AttackType::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned()
                    .col(
                        ColumnDef::new(AttackTypesGladiators::GladiatorId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_gladiator_id")
                            .from(
                                AttackTypesGladiators::Table,
                                AttackTypesGladiators::GladiatorId,
                            )
                            .to(Gladiator::Table, Gladiator::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AttackTypesGladiators::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum AttackTypesGladiators {
    Table,
    Id,
    GladiatorId,
    AttackTypeId,
}
