use specs::Entity;
use uuid::Uuid;

pub struct LogoutEvents(pub Vec<LogoutEvent>);

pub struct LogoutEvent {
    pub client_id: Uuid,
    pub entity: Entity,
}
