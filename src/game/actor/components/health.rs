use specs::{Component, VecStorage};

pub struct Health {
    pub max_health : i16,
    pub health : i16,
}

impl Health {
    pub fn add(&mut self, amount : i16) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}
