use specs::{Component, VecStorage};

#[derive(Default)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

impl Health {
    pub fn add(&mut self, amount: f32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn reduce(&mut self, amount: f32) {
        self.health -= amount;
        if self.health < 0f32 {
            self.health = 0f32;
        }
    }
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}
