use specs::Entity;

pub struct FightEvent {
    pub initiator: Entity,
}

#[derive(Default)]
pub struct FightEvents(pub Vec<FightEvent>);
