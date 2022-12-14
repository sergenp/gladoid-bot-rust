use super::attack_type::Model as AttackTypeModel;
use sea_orm::{entity::prelude::*, DatabaseConnection};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "gladiator")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub attack: i32,
    pub defence: i32,
    pub level: i32,
    #[sea_orm(ignore)]
    pub health: f32,
    #[sea_orm(ignore)]
    pub attacks: Vec<AttackTypeModel>,
}

impl Model {
    pub fn calculate_secondary_stats(mut self) -> Self {
        self.health = ((self.defence as f32 * 5.0) + 5.0) * 0.5;
        return self;
    }

    pub async fn populate_attacks(mut self, db: &DatabaseConnection) -> Self {
        self.attacks = Entity::find_attacks(self.id, db).await;
        return self;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::attack_types_gladiators::Entity")]
    AttackTypesGladiators,
}

impl Related<super::attack_types_gladiators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttackTypesGladiators.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    /// Finds the attacks for the given gladiator id,
    /// by joining the m2m table between gladiator and attack_types
    /// useful for a varius of reasons
    pub async fn find_attacks(gladiator_id: i32, db: &DatabaseConnection) -> Vec<AttackTypeModel> {
        return super::attack_type::Entity::find()
            .inner_join(super::attack_types_gladiators::Entity)
            .filter(super::attack_types_gladiators::Column::GladiatorId.eq(gladiator_id))
            .all(db)
            .await
            .unwrap();
    }
}

pub trait Attackable<T> {
    /// Calculates an attack damage with given attack_id.
    fn calculate_attack_damage(&self, attack_id: i32) -> f32;

    /// Every attackable trait has take damage function.
    /// If you're attackable, you can attack and take damage
    fn take_damage(&mut self, pure_damage: f32) -> f32;
}

impl Attackable<Model> for Model {
    fn calculate_attack_damage(&self, attack_id: i32) -> f32 {
        let attack_type: &AttackTypeModel = self.find_attack(attack_id);
        return (self.attack + attack_type.attack_damage) as f32;
    }

    fn take_damage(&mut self, pure_damage: f32) -> f32 {
        let damage: f32 = pure_damage - self.defence as f32;
        // let damage: f32 = pure_damage / (self.defence as f32 / pure_damage).powf(2.0);
        self.health = {
            let hp = self.health - damage;
            if hp > 0.0 {
                hp
            } else {
                0.0
            }
        };

        println!(
            "{} took {} damage. {} has {} hp left",
            self.name, damage, self.name, self.health
        );

        return damage;
    }
}

impl Model {
    fn find_attack(&self, attack_id: i32) -> &AttackTypeModel {
        return self
            .attacks
            .iter()
            .find(|el| -> bool { el.id == attack_id })
            .expect("Couldn't find attack");
    }
}
