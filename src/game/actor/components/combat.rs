use specs::{Component, VecStorage};

pub struct Combat {
    pub attack: f32,
    pub defense: f32,
}

impl Component for Combat {
    type Storage = VecStorage<Self>;
}
