use specs::{Component, VecStorage};

pub struct Kind {
    pub is_monster: bool,
}

impl Component for Kind {
    type Storage = VecStorage<Self>;
}
