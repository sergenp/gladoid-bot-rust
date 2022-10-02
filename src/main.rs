mod game;

use game::gladiator::player::Gladiator as Gladiator;
use game::gladiator::player::Attackable;
use game::gladiator::attack::AttackType as AttackType;

#[async_std::main]
async fn main() {

    let player: Gladiator = Gladiator{
        name: "Sergen".to_string(), 
        attacks : &mut Vec::<AttackType>::new(),
        attack_stat: 5, 
        defense_stat: 5, 
        health: 20 
    };

    let enemy: Gladiator = Gladiator{
        name:"Quanna".to_string(),
        attacks:&mut Vec::<AttackType>::new(),
        attack_stat: 10,
        defense_stat : 6,
        health: 20
    };

    let attack_type: AttackType = AttackType{
        id:1,
        name:"main".to_string(),
        attack_damage:5
    };

    let attack_type2: AttackType = AttackType{
        id:2,
        name:"main2".to_string(),
        attack_damage:10
    };


    let attack_type_enemy: AttackType = attack_type.clone();
    
    player.attacks.push(attack_type);
    player.attacks.push(attack_type2);
    enemy.attacks.push(attack_type_enemy);

    println!("{}", enemy.attacks[0].name);
    player.attack(enemy, 1);
}
