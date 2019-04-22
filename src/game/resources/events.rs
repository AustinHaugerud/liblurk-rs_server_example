use specs::World;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////

pub struct ConnectEvent {
    pub id: Uuid,
}

pub struct DisconnectEvent {
    pub id: Uuid,
}

pub struct MessageEvent {
    pub initiator: Uuid,
    pub target: String,
    pub content: String,
}

pub struct ChangeRoomEvent {
    pub initiator: Uuid,
    pub room_number: u16,
}

pub struct FightEvent {
    pub initiator: Uuid,
}

pub struct PvpFightEvent {
    pub initiator: Uuid,
    pub target: String,
}

pub struct LootEvent {
    pub initiator: Uuid,
    pub target: String,
}

pub struct StartEvent {
    pub initiator: Uuid,
}

pub struct CharacterEvent {
    pub initiator: Uuid,
    pub name: String,
    pub attack: u16,
    pub defense: u16,
    pub regen: u16,
    pub description: String,
}

pub struct LeaveEvent {
    pub initiator: Uuid,
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ConnectEvents(pub Vec<ConnectEvent>);

#[derive(Default)]
pub struct DisconnectEvents(pub Vec<DisconnectEvent>);

#[derive(Default)]
pub struct MessageEvents(pub Vec<MessageEvent>);

#[derive(Default)]
pub struct ChangeRoomEvents(pub Vec<ChangeRoomEvent>);

#[derive(Default)]
pub struct FightEvents(pub Vec<FightEvent>);

#[derive(Default)]
pub struct PvpFightEvents(pub Vec<PvpFightEvent>);

#[derive(Default)]
pub struct LootEvents(pub Vec<LootEvent>);

#[derive(Default)]
pub struct StartEvents(pub Vec<StartEvent>);

#[derive(Default)]
pub struct CharacterEvents(pub Vec<CharacterEvent>);

#[derive(Default)]
pub struct LeaveEvents(pub Vec<LeaveEvent>);

pub fn register_event_stores_to_world(world: &mut World) {
    world.add_resource(ConnectEvents::default());
    world.add_resource(DisconnectEvents::default());
    world.add_resource(MessageEvents::default());
    world.add_resource(ChangeRoomEvents::default());
    world.add_resource(FightEvents::default());
    world.add_resource(PvpFightEvents::default());
    world.add_resource(LootEvents::default());
    world.add_resource(StartEvents::default());
    world.add_resource(CharacterEvents::default());
    world.add_resource(LeaveEvents::default());
}

///////////////////////////////////////////////////////////////////////////////
