use specs::{Component, VecStorage};
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct Id(Uuid);

impl Component for Id {
    type Storage = VecStorage<Self>;
}
