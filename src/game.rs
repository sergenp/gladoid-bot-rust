use std::cell::{Cell, RefCell};

use crate::entity::gladiator::Attackable;

use super::entity::gladiator::Model as Gladiator;

pub struct GladiatorGame{
    pub current_turn : usize,
    pub players : [Gladiator;2]
}
impl GladiatorGame{
    pub fn create_game(player1:Gladiator, player2:Gladiator) -> GladiatorGame {
        return GladiatorGame{current_turn:0, players: [player1, player2]};
    }

    pub fn game_loop(&mut self) {
        loop {
            let current_player_index = self.current_turn % self.players.len();
            let next_player_index = (self.current_turn+1) % self.players.len();
            
            let current_player = &self.players[current_player_index];
            let pure_damage = current_player.calculate_attack_damage(current_player.attacks[0].id);
            
            let next_player = &mut self.players[next_player_index];
            next_player.take_damage(pure_damage);

            if next_player.health <= 0.0 {
                break;
            }

            self.current_turn += 1;
        }
    }
}
