use specs::Entity;

pub struct PvpFightEvent {
    pub target: String,
    pub initiator: Entity,
}

#[derive(Default)]
pub struct PvpFightEvents(pub Vec<PvpFightEvent>);
