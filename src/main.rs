#![allow(unused_imports)]
#![allow(unused_variables)]
// #![allow(dead_code)]

mod entity;
mod game;

use entity::attack_type::ActiveModel as AttackTypeActiveModel;
use entity::attack_type::Entity as AttackTypeEntity;
use entity::attack_types_gladiators::ActiveModel;
use entity::attack_types_gladiators::Entity as AttackTypesGladiatorsEntity;
use entity::gladiator::ActiveModel as GladiatorActiveModel;
use entity::gladiator::Entity as GladiatorEntity;

use game::GladiatorGame;

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
        .expect("Database error")
        .expect("Didn't find any Model")
        .calculate_secondary_stats()
        .populate_attacks(&db).await;

    let gladiator_2 = GladiatorEntity::find_by_id(2)
        .one(&db)
        .await
        .expect("Database error")
        .expect("Didn't find any Model")
        .calculate_secondary_stats()
        .populate_attacks(&db).await;
    
    // let gladiator_2 = match gladiator_2 {
    //     Some(x) =>  x.calculate_secondary_stats().populate_attacks(&db).await,
    //     None => panic!(),
    // };

    println!("Gladiator 1 health: {}, level: {}, attack: {}, defence: {}", gladiator.health, gladiator.level, gladiator.attack, gladiator.defence);
    println!("Gladiator 1 health: {}, level: {}, attack: {}, defence: {}", gladiator_2.health, gladiator_2.level, gladiator_2.attack, gladiator_2.defence);

    // player_1 = Gladiator

    let mut game = GladiatorGame::create_game(gladiator, gladiator_2);

    game.game_loop();

    // println!("Gladiator {} level: {}", game.players[0].name, game.players[0].level);
    // println!("Gladiator {} level: {}", game.players[1].name, game.players[1].level);

    
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
