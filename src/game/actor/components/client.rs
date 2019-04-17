use specs::{Component, VecStorage};
use uuid::Uuid;

pub struct Client {
    pub id: Uuid,
}

impl Component for Client {
    type Storage = VecStorage<Self>;
}
