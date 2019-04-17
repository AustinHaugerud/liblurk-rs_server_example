use specs::Entity;

pub struct LootEvent {
    pub target: String,
    pub looter: Entity,
}

#[derive(Default)]
pub struct LootEvents(pub Vec<LootEvent>);
