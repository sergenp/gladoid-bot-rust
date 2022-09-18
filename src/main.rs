mod game;

use game::gladiator::player::Gladiator as Gladiator;

fn main() {
    let player: Gladiator = Gladiator{
        name: "Sergen".to_string(), 
        attack_stat: 5, 
        defense_stat: 5, 
        health: 20 
    };

    let enemy: Gladiator = Gladiator{
        name:"Quanna".to_string(),
        attack_stat: 10,
        defense_stat : 6,
        health: 20
    };
    
    player.attack(enemy);
}
