use specs::{Component, VecStorage};

pub struct Gold(u16);

impl Component for Gold {
    type Storage = VecStorage<Self>;
}
