use specs::{Component, VecStorage};

pub struct Number(pub u16);

impl Component for Number {
    type Storage = VecStorage<Self>;
}
