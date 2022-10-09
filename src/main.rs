#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod entity;
mod game;

use entity::attack_type::ActiveModel as AttackTypeActiveModel;
use entity::attack_type::Entity as AttackTypeEntity;
use entity::attack_types_gladiators::ActiveModel as AttackTypesGladiatorsActiveModel;
use entity::attack_types_gladiators::Entity as AttackTypesGladiatorsEntity;

use entity::gladiator::ActiveModel as GladiatorActiveModel;
use entity::gladiator::Entity as GladiatorEntity;

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::ModelTrait;
use sea_orm::QueryTrait;
use sea_orm::{prelude::Uuid, ActiveModelTrait, Database, DatabaseBackend, EntityTrait};

#[async_std::main]
async fn main() {
    let db = Database::connect(
        "sqlite:///D:/sergen/Masaüstü/Programming/rust/gladoid-bot/migration/gladoidbot.sqlite",
    )
    .await
    .unwrap();

    let gladiator = GladiatorEntity::find_by_id(1)
        .one(&db)
        .await
        .unwrap()
        .expect("Didn't find any Model with id 1");

    let attacks = GladiatorEntity::find_attacks(gladiator.id, db).await;

    for attack in attacks {
        println!("{}", attack.name);
    }
    // let attacks = gladiator
    //     .find_related(AttackTypesGladiatorsEntity)
    //     .find_with_related(AttackTypeEntity)
    //     .all(&db)
    //     .await
    //     .unwrap();
    // .build(DatabaseBackend::Sqlite);

    // println!("{}", attacks);

    // for attack in attacks {
    //     println!("{}", attack.1.len());
    //     for attack_type in attack.1 {
    //         println!("{}", attack_type.name);
    //         println!("{}", attack_type.attack_damage);
    //     }
    // }

    // let gladiator = gladiator.insert(&db).await.unwrap();

    // let attack_type = AttackTypeActiveModel {
    //     attack_damage: Set(5),
    //     name: Set("Slash".to_owned()),
    //     ..Default::default()
    // };

    // let attack_type = attack_type.insert(&db).await.unwrap();

    // let attack_types_gladiators = AttackTypesGladiatorsActiveModel {
    //     attack_type_id: Set(attack_type.id),
    //     gladiator_id: Set(gladiator.id),
    //     ..Default::default()
    // };

    // let attack_types_gladiators = attack_types_gladiators.insert(&db).await.unwrap();
}
