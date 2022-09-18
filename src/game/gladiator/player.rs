pub struct Gladiator {
    pub name:String,
    pub attack_stat:i32,
    pub defense_stat:i32,   
    pub health:i32,
}

impl Gladiator {
    pub fn attack(&self, mut enemy: Gladiator) {
        let damage: i32 = Gladiator::take_damage(&mut enemy, self.attack_stat);
        println!("{} took {} damage. {} has {} hp left", enemy.name, damage, enemy.name, enemy.health);
    }

    pub fn take_damage(&mut self, pure_damage:i32) -> i32{
        let mut damage: i32 = pure_damage - self.defense_stat;
        damage = if damage > 0 {damage} else {0};
        self.health -= damage;
        damage
    }
}
