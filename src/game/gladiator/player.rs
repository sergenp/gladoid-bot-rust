use crate::game;

use game::gladiator::attack::AttackType;

pub struct Gladiator<'a> {
    pub name: String,
    pub attacks: &'a mut Vec<AttackType>,
    pub attack_stat: i32,
    pub defense_stat: i32,
    pub health: i32,
}

pub trait Attackable<T> {
    fn attack(&self, enemy: T, attack_id: u32);
    fn take_damage(&mut self, pure_damage: i32) -> i32;
}

impl Attackable<Gladiator<'_>> for Gladiator<'_> {
    fn attack(&self, mut enemy: Gladiator, attack_id: u32) {
        let attack_type: &AttackType = self
            .find_attack(attack_id)
            .expect("Attack with given id is not found");
        let pure_damage: i32 = self.attack_stat + attack_type.attack_damage;
        let damage: i32 = Attackable::take_damage(&mut enemy, pure_damage);
        println!(
            "{} took {} damage. {} has {} hp left",
            enemy.name, damage, enemy.name, enemy.health
        );
    }
    fn take_damage(&mut self, pure_damage: i32) -> i32 {
        let mut damage: i32 = pure_damage - self.defense_stat;
        damage = if damage > 0 { damage } else { 0 };
        self.health -= damage;
        damage
    }
}

impl<'a> Gladiator<'a> {
    pub fn find_attack(&self, attack_id: u32) -> Option<&AttackType> {
        return self
            .attacks
            .iter()
            .find(|el: &&AttackType| -> bool { el.id == attack_id });
    }
}
