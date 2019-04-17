use specs::{Component, VecStorage};

pub struct Behaviour {
    hostile: bool, // Will attack players on sight.
    pursues: bool, // Will try to follow players if they leave the location.
    greedy: bool,  // Will loot corpses for gold.
}

impl Component for Behaviour {
    type Storage = VecStorage<Self>;
}
