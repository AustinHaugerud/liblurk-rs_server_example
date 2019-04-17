use specs::{Component, VecStorage};

pub struct Gold(pub u16);

impl Component for Gold {
    type Storage = VecStorage<Self>;
}