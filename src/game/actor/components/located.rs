use specs::{Component, VecStorage};

pub struct Located {

}

impl Component for Located {
    type Storage = VecStorage<Self>;
}
