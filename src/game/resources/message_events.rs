use specs::Entity;

#[derive(Default)]
pub struct MessageEvents(pub Vec<MessageEvent>);

pub struct MessageEvent {
    pub sender: Entity,
    pub target: String,
    pub content: String,
}
