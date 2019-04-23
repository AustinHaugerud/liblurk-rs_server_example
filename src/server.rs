use game::load::constants_loader::ConstantsLoader;
use game::resources::character_prep::CharacterPrep;
use game::resources::global_name_registry::GlobalNameRegistry;
use game::resources::id_name_mapping::IdNameMapping;
use game::resources::move_task::MoveTasks;
use game::resources::start_location::StartLocation;
use game::resources::start_registry::StartRegistry;
use game::types::GameConstants;
use liblurk::protocol::protocol_message::{ChangeRoom, Character, Loot, Message, PvpFight};
use liblurk::server::callbacks::ServerCallbacks;
use liblurk::server::context::ServerEventContext;
use liblurk::server::server_access::WriteContext;
use specs::{DispatcherBuilder, World};

pub struct Server {
    world: World,
}

impl Server {
    pub fn new() -> Result<Server, String> {
        use game::components;
        use game::load;
        use game::resources;

        let constants_loader = ConstantsLoader::new("data/constants.ron");
        let constants = constants_loader.load_constants()?;

        let mut world = World::new();

        components::entity::register_components_to_world(&mut world);
        components::location::register_components_to_world(&mut world);
        resources::events::register_event_stores_to_world(&mut world);
        world.add_resource::<Option<WriteContext>>(None);
        world.add_resource(GlobalNameRegistry::default());
        world.add_resource(IdNameMapping::default());
        world.add_resource(CharacterPrep::default());
        world.add_resource(constants.clone());
        world.add_resource(StartRegistry::default());
        world.add_resource(MoveTasks::default());

        let locations_loader = load::location_loader::LocationLoader::new("data/locations");
        let start_loc = load::location_loader::add_locations_to_world(
            locations_loader,
            &mut world,
            &constants.starting_location,
        )?;
        world.add_resource(StartLocation(Some(start_loc)));

        Ok(Server { world })
    }

    pub fn soft_register_write_context(&mut self, write_context: WriteContext) {
        if self.world.read_resource::<Option<WriteContext>>().is_none() {
            self.world.add_resource(Some(write_context))
        }
    }
}

impl ServerCallbacks for Server {
    fn on_connect(&mut self, context: &ServerEventContext) {
        println!("New connection.");
        use game::resources::events::{ConnectEvent, ConnectEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut connect_events = self.world.write_resource::<ConnectEvents>();
        let event = ConnectEvent {
            id: *context.get_client_id(),
        };
        connect_events.0.push(event);
    }

    fn on_disconnect(&mut self, context: &ServerEventContext) {
        println!("Disconnection.");
        use game::resources::events::{DisconnectEvent, DisconnectEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut disconnect_events = self.world.write_resource::<DisconnectEvents>();
        let event = DisconnectEvent {
            id: *context.get_client_id(),
        };
        disconnect_events.0.push(event);
    }

    fn on_message(&mut self, context: &ServerEventContext, message: &Message) {
        println!("Message received!.");
        use game::resources::events::{MessageEvent, MessageEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut message_events = self.world.write_resource::<MessageEvents>();
        let event = MessageEvent {
            initiator: *context.get_client_id(),
            target: message.receiver.clone(),
            content: message.message.clone(),
        };
        message_events.0.push(event);
    }

    fn on_change_room(&mut self, context: &ServerEventContext, change_room: &ChangeRoom) {
        println!("Change room received!");
        use game::resources::events::{ChangeRoomEvent, ChangeRoomEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut change_room_events = self.world.write_resource::<ChangeRoomEvents>();
        let event = ChangeRoomEvent {
            initiator: *context.get_client_id(),
            room_number: change_room.room_number,
        };
        change_room_events.0.push(event);
    }

    fn on_fight(&mut self, context: &ServerEventContext) {
        println!("Fight received!");
        use game::resources::events::{FightEvent, FightEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut fight_events = self.world.write_resource::<FightEvents>();
        let event = FightEvent {
            initiator: *context.get_client_id(),
        };
        fight_events.0.push(event);
    }

    fn on_pvp_fight(&mut self, context: &ServerEventContext, pvp_fight: &PvpFight) {
        println!("Pvp fight received.");
        use game::resources::events::{PvpFightEvent, PvpFightEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut pvp_fight_events = self.world.write_resource::<PvpFightEvents>();
        let event = PvpFightEvent {
            initiator: *context.get_client_id(),
            target: pvp_fight.target.clone(),
        };
        pvp_fight_events.0.push(event);
    }

    fn on_loot(&mut self, context: &ServerEventContext, loot: &Loot) {
        println!("Loot received.");
        use game::resources::events::{LootEvent, LootEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut loot_events = self.world.write_resource::<LootEvents>();
        let event = LootEvent {
            initiator: *context.get_client_id(),
            target: loot.target.clone(),
        };
        loot_events.0.push(event);
    }

    fn on_start(&mut self, context: &ServerEventContext) {
        println!("Start received!");
        use game::resources::events::{StartEvent, StartEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut start_events = self.world.write_resource::<StartEvents>();
        let event = StartEvent {
            initiator: *context.get_client_id(),
        };
        start_events.0.push(event);
    }

    fn on_character(&mut self, context: &ServerEventContext, character: &Character) {
        println!("Character received.");
        use game::resources::events::{CharacterEvent, CharacterEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut character_events = self.world.write_resource::<CharacterEvents>();
        let event = CharacterEvent {
            initiator: *context.get_client_id(),
            name: character.player_name.clone(),
            attack: character.attack,
            defense: character.defense,
            regen: character.regeneration,
            description: character.description.clone(),
        };
        character_events.0.push(event);
    }

    fn on_leave(&mut self, context: &ServerEventContext) {
        println!("Leave received!");
        use game::resources::events::{LeaveEvent, LeaveEvents};

        self.soft_register_write_context(context.get_write_context());

        let mut leave_events = self.world.write_resource::<LeaveEvents>();
        let event = LeaveEvent {
            initiator: *context.get_client_id(),
        };
        leave_events.0.push(event);
    }

    fn update(&mut self, context: WriteContext) {
        use game::systems;

        self.soft_register_write_context(context);

        let mut dispatcher = systems::get_dispatcher();

        dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
    }
}
