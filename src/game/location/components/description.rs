use specs::{Component, VecStorage};

pub struct Description(String);

impl Component for Description {
    type Storage = VecStorage<Self>;
}
