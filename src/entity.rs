pub struct Entity {
    pub name: String,
    pub attack: u16,
    pub defense: u16,
    pub regen: u16,
    pub health: i16,
    pub gold: u16,
    pub location: u16, // the room number
    pub alive: bool,
    pub monster: bool,
    pub desc: String,
    pub base_health : i16,
}

impl Entity {
    // The more gold a player has, the more their stats are scaled
    pub fn get_gold_skill_multiplier(&self) -> f32 {
        let boost = (self.gold as f32 * 0.01f32);
        match self.monster {
            true => 1f32,
            false => 1f32 + boost,
        }
    }

    pub fn get_effective_attack(&self) -> u16 {
        (self.attack as f32 * self.get_gold_skill_multiplier()) as u16
    }

    pub fn get_effective_defense(&self) -> u16 {
        (self.defense as f32 * self.get_gold_skill_multiplier()) as u16
    }

    pub fn get_effective_regen(&self) -> u16 {
        (self.defense as f32 * self.get_gold_skill_multiplier()) as u16
    }

    pub fn get_max_health(&self) -> i16 {
        if self.monster {
            return self.base_health;
        }

        (self.base_health as f32 * self.get_gold_skill_multiplier()) as i16
    }

    pub fn regen(&mut self) {
        if self.alive {
            let points = (self.get_effective_regen() as f32 / 10f32) as i16;
            println!("{} heal {} points.", self.name, points);
            self.health = (self.health + points).min(self.get_max_health());
            println!("Health is now {}.", self.health);
        }
    }
}
