use specs::{Component, Entity, VecStorage};
use game::actor::stat::Stat;

pub struct Purchase {
    pub entity : Entity,
    pub amount : u16,
    pub target : Stat,
}

impl Component for Purchase {
    type Storage = VecStorage<Self>;
}
