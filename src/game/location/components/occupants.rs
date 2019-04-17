use specs::{Component, Entity, VecStorage};
use std::collections::HashMap;

pub struct Occupants {
    pub tenants: HashMap<String, Entity>,
}

impl Component for Occupants {
    type Storage = VecStorage<Self>;
}
