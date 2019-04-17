use specs::{Component, Entity, VecStorage};

pub struct Adjacencies(pub Vec<Entity>);

impl Component for Adjacencies {
    type Storage = VecStorage<Self>;
}
