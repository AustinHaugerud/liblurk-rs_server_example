use specs::{Component, VecStorage};

pub struct Name(String);

impl Component for Name {
    type Storage = VecStorage<Self>;
}