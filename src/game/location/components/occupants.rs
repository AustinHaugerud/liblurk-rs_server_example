use specs::{Component, VecStorage};
use game::actor::components::id::Id;

pub struct Occupants {
    ids : Vec<Id>
}

impl Component for Occupants {
    type Storage = VecStorage<Self>;
}
