use specs::{Component, VecStorage};

pub struct Combat {
    pub attack : u16,
    pub defense : u16,
}

impl Component for Combat {
    type Storage = VecStorage<Self>;
}
