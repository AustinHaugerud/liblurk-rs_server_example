extern crate liblurk;
extern crate rand;
extern crate uuid;

/*#[macro_use]
extern crate nickel;*/

extern crate specs;

extern crate clap;

mod game;

use uuid::Uuid;

use liblurk::protocol::protocol_message::*;
use liblurk::server::server::{
    LurkServerError, Server, ServerCallbacks, ServerEventContext, UpdateContext,
};

use specs::prelude::*;

use game::resources::change_room_events::ChangeRoomEvent;
use game::resources::change_room_events::ChangeRoomEvents;
use game::resources::fight_events::FightEvent;
use game::resources::fight_events::FightEvents;
use game::resources::loot_events::LootEvent;
use game::resources::loot_events::LootEvents;
use game::resources::pvp_fight_events::PvpFightEvent;
use game::resources::pvp_fight_events::PvpFightEvents;
use std::collections::HashMap;

use game::resources::character_creation::CharacterCreateItem;
use game::resources::character_creation::CharacterCreation;
use game::resources::logout_events::LogoutEvent;
use game::resources::logout_events::LogoutEvents;
use game::resources::message_events::MessageEvent;
use game::resources::message_events::MessageEvents;
use clap::App;
use clap::Arg;
use std::net::SocketAddr;
use std::time::Duration;
use game::resources::feedback::FeedbackItems;

struct ExampleServer {
    world: World,
    clients_to_entities: HashMap<Uuid, Entity>,
}

impl ExampleServer {
    fn new() -> ExampleServer {
        ExampleServer {
            world: ExampleServer::build_world(),
            clients_to_entities: HashMap::new(),
        }
    }

    fn build_world() -> World {
        use game::actor::components::register_actor_components;
        use game::location::components::register_location_components;

        let mut world = World::new();
        world.add_resource(game::resources::change_room_events::ChangeRoomEvents::default());
        world.add_resource(game::resources::fight_events::FightEvents::default());
        world.add_resource(game::resources::loot_events::LootEvents::default());
        world.add_resource(game::resources::pvp_fight_events::PvpFightEvents::default());
        world.add_resource(game::resources::character_creation::CharacterCreation::default());
        world.add_resource(game::resources::message_events::MessageEvents::default());

        register_actor_components(&mut world);
        register_location_components(&mut world);

        world
    }

    fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
        use game::actor::systems::regeneration_system::RegenSystem;
        use game::systems::{
            change_room_system::ChangeRoomSystem, loot_system::LootSystem,
            player_fight_system::PlayerFightSystem,
        };

        DispatcherBuilder::new()
            .with(RegenSystem, "regen_system", &[])
            .with(ChangeRoomSystem, "change_room_system", &[])
            .with(LootSystem, "loot_system", &[])
            .with(PlayerFightSystem, "player_fight_system", &[])
            .build()
    }
}

impl ServerCallbacks for ExampleServer {
    fn on_connect(&mut self, context: &mut ServerEventContext) -> LurkServerError {
        println!("Connection made.");
        Ok(())
    }

    fn on_disconnect(&mut self, client_id: &Uuid) {
        self.on_leave(client_id)
            .expect("Failed on_leave in on_disconnect.");
    }

    fn on_message(
        &mut self,
        context: &mut ServerEventContext,
        message: &Message,
    ) -> LurkServerError {
        if let Some(entity) = self.clients_to_entities.get(&context.get_client_id()) {
            let mut message_events = self.world.write_resource::<MessageEvents>();

            message_events.0.push(MessageEvent {
                sender: *entity,
                target: message.receiver.clone(),
                content: message.message.clone(),
            });
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You are not in the world yet.".to_string(),
            ).expect("Bug: Invalid error packet."));
        }

        Ok(())
    }

    fn on_change_room(
        &mut self,
        context: &mut ServerEventContext,
        change_room: &ChangeRoom,
    ) -> LurkServerError {
        if let Some(player_entity) = self.clients_to_entities.get(&context.get_client_id()) {
            let event = ChangeRoomEvent {
                mover: *player_entity,
                target_room: change_room.room_number,
            };

            self.world
                .write_resource::<ChangeRoomEvents>()
                .0
                .push(event);
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You are not in the world yet.".to_string(),
            ).expect("Bug: Invalid error packet."));
        }

        Ok(())
    }

    fn on_fight(&mut self, context: &mut ServerEventContext, _: &Fight) -> LurkServerError {
        if let Some(player_entity) = self.clients_to_entities.get(&context.get_client_id()) {
            let event = FightEvent {
                initiator: *player_entity,
            };

            self.world.write_resource::<FightEvents>().0.push(event);
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You are not in the world yet.".to_string(),
            ).expect("Bug: Invalid error packet."))
        }

        Ok(())
    }

    fn on_pvp_fight(
        &mut self,
        context: &mut ServerEventContext,
        pvpfight: &PvpFight,
    ) -> LurkServerError {
        if let Some(player_entity) = self.clients_to_entities.get(&context.get_client_id()) {
            let event = PvpFightEvent {
                target: pvpfight.target.clone(),
                initiator: *player_entity,
            };

            self.world.write_resource::<PvpFightEvents>().0.push(event);
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You are not in the world yet.".to_string(),
            ).expect("Bug; Invalid error packet."));
        }

        Ok(())
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, loot: &Loot) -> LurkServerError {
        if let Some(player_entity) = self.clients_to_entities.get(&context.get_client_id()) {
            let event = LootEvent {
                target: loot.target.clone(),
                looter: *player_entity,
            };

            self.world.write_resource::<LootEvents>().0.push(event);
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You are not in the world yet.".to_string(),
            ).expect("Bug: Invalid error packet."));
        }

        Ok(())
    }

    fn on_start(&mut self, context: &mut ServerEventContext, _: &Start) -> LurkServerError {
        let mut character_creation = self.world.write_resource::<CharacterCreation>();
        if let Some(submission) = character_creation.0.get_mut(&context.get_client_id()) {
            submission.submitted = true;
        } else {
            context.enqueue_message_this(Error::not_ready(
                "You need to submit a valid character before starting."
                    .to_string()
            ).expect("Bug: Invalid error packet."));
        }
        Ok(())
    }

    fn on_character(
        &mut self,
        context: &mut ServerEventContext,
        character: &Character,
    ) -> LurkServerError {
        if !self
            .clients_to_entities
            .contains_key(&context.get_client_id())
        {
            let mut character_creation = self.world.write_resource::<CharacterCreation>();
            character_creation.0.insert(
                context.get_client_id(),
                CharacterCreateItem {
                    client_id: context.get_client_id(),
                    character_packet: character.clone(),
                    submitted: false,
                },
            );
        } else {
            context.enqueue_message_this(
                Error::other("You have already started with a character.".to_string())
                    .expect("Bug: Invalid error packet."),
            )
        }

        Ok(())
    }

    fn on_leave(&mut self, client_id: &Uuid) -> LurkServerError {
        if let Some(entity) = self.clients_to_entities.get(client_id) {
            let mut logout_events = self.world.write_resource::<LogoutEvents>();
            logout_events.0.push(LogoutEvent {
                client_id: client_id.clone(),
                entity: *entity,
            });
        }

        Ok(())
    }

    fn update(&mut self, context: &UpdateContext) {
        let mut dispatcher = Self::build_dispatcher();
        dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        let mut feedback = self.world.write_resource::<FeedbackItems>();

        for item in feedback.0.drain(..) {
            context.enqueue_message_dyn(item.packet, item.send_target);
        }
    }
}

fn main() {

    let matches = App::new("ExampleServer[liblurk-rs]")
        .version("2.0")
        .author("Austin Jenkins")
        .about("Example server using liblurk-rs")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("SOCKET ADDRESS")
            .help("Set server socket address")
            .takes_value(true)
            .required(true))
        .get_matches();

    let addr_str = matches.value_of("address").expect("Missing address argument.");

    if let Ok(addr) = addr_str.parse::<SocketAddr>() {
        let example_server = Box::new(ExampleServer::new());
        if let Ok(mut lurk_server) = Server::create(addr, Duration::from_secs(60), example_server) {
            lurk_server.start();
        }
        else {
            println!("Failed to start server.");
        }
    }
    else {
        println!("Invalid server address.");
    }

}
