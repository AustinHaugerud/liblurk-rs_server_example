use specs::Entity;

pub struct ChangeRoomEvent {
    pub mover: Entity,
    pub target_room: u16,
}

#[derive(Default)]
pub struct ChangeRoomEvents(pub Vec<ChangeRoomEvent>);
