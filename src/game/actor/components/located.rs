use specs::{Component, Entity, VecStorage};

pub struct Located {
    pub room: Entity,
}

impl Component for Located {
    type Storage = VecStorage<Self>;
}
