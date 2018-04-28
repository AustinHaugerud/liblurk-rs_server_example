extern crate liblurk;
extern crate uuid;

mod map;
mod entity;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use uuid::Uuid;

use liblurk::server::server::{Server, ServerCallbacks, ServerEventContext};
use liblurk::protocol::send::LurkSendChannel;
use liblurk::protocol::protocol_message::*;

use map::*;
use entity::*;

const INITIAL_POINTS: u16 = 100;
const STAT_LIMIT: u16 = 100;

pub fn get_game_packet() -> Game {
    Game {
        initial_points: INITIAL_POINTS,
        stat_limit: STAT_LIMIT,
        description: String::from("You find yourself in an uneventful boring dungeon."),
    }
}

struct Player {
    entity_info: Entity,
    ready: bool,
    started: bool,
}

struct ExampleServer {
    players: HashMap<Uuid, Player>,
    map: Map,
}

impl ExampleServer {
    fn new() -> ExampleServer {
        let mut map_builder = MapBuilder::new();

        let entry_room_id =
            map_builder.register_room("Entry Room", "This room seems to be the entrance.");
        let basement_id =
            map_builder.register_room("Basement", "It's very dark, and there seems to be some gross old canned food. It looks like there's a dumbweighter to the attic.");
        let parlor_id =
            map_builder.register_room("Parlor", "There's a mess of old furniture and music.");
        let attic_id =
            map_builder.register_room("Attic", "Eek! There's some big spiders up here! There seems to be a dumbweighter to the basement.");

        map_builder
            .link_rooms(entry_room_id, parlor_id)
            .expect("Failed to link entry room and parlor.");
        map_builder
            .link_rooms(parlor_id, attic_id)
            .expect("Failed to link parlor and attic.");
        map_builder
            .link_rooms(parlor_id, basement_id)
            .expect("Failed to link parlor and basement.");
        map_builder
            .link_rooms(attic_id, basement_id)
            .expect("Failed to link attic and basement.");

        let map = map_builder.complete();

        ExampleServer {
            players: HashMap::new(),
            map,
        }
    }

    fn get_player_id_by_name(&self, search_name: &String) -> Option<Uuid> {
        for (id, player) in self.players.iter() {
            if search_name.eq(&player.name) {
                return Some(id.clone());
            }
        }
        None
    }
}

impl ServerCallbacks for ExampleServer {
    fn on_connect(&mut self, context: &mut ServerEventContext) {
        if context
            .get_send_channel()
            .write_message(get_game_packet())
            .is_err()
        {
            println!("Error: Failed to send game message to incoming client.");
        } else {
            println!("Connection made!");
            self.players.insert(
                &client_id,
                Player {
                    entity_info: Entity {
                        name: String::new(),
                        attack: 0,
                        defense: 0,
                        regen: 0,
                        gold: 0,
                        location: 0,
                        alive: false,
                        monster: false,
                    },
                    ready: false,
                    started: false,
                },
            );
        }
    }

    fn on_disconnect(&mut self, client_id: &Uuid) {
        self.players.remove(client_id);
    }

    fn on_message(&mut self, context: &mut ServerEventContext, message: &Message) {
        println!("Got message packet.");
        if let Some(id) = self.get_player_id_by_name(&message.receiver) {
            let op = context.get_client(&id).expect("Could not find client.");
            let mut client = op.lock().unwrap();
            client.get_send_channel().write_message_ref(message);
        } else {
            println!("Got message, didn't know who to send it to.");
        }
    }

    fn on_change_room(&mut self, context: &mut ServerEventContext, change_room: &ChangeRoom) {
        unimplemented!()
    }

    fn on_fight(&mut self, context: &mut ServerEventContext, fight: &Fight) {
        unimplemented!()
    }

    fn on_pvp_fight(&mut self, context: &mut ServerEventContext, pvp_fight: &PvpFight) {
        unimplemented!()
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, loot: &Loot) {
        unimplemented!()
    }

    fn on_start(&mut self, context: &mut ServerEventContext, start: &Start) {}

    fn on_character(&mut self, context: &mut ServerEventContext, character: &Character) {
        println!("Got character message.");

        if character.attack + character.defense + character.regeneration > INITIAL_POINTS {
            context.get_send_channel().write_message(
                Error::stat_error("Invalid amount of stat points spent.".to_string()).unwrap(),
            );
            return;
        }

        if character.attack > STAT_LIMIT || character.defense > STAT_LIMIT
            || character.regeneration > STAT_LIMIT
        {
            context.get_send_channel().write_message(
                Error::stat_error("One or more attributes were set too high.".to_string()).unwrap(),
            );
            return;
        }

        if let Some(player) = self.players.get(&context.get_client_id()) {
            if !player.started {
                context
                    .get_send_channel()
                    .write_message(Accept::new(CHARACTER_TYPE))
                    .expect("Failed to send accept character.");
            } else {
                context.get_send_channel().write_message(
                    Error::other("Your stats cannot be edited at this time.".to_string()).unwrap(),
                );
            }
        }
    }

    fn on_leave(&mut self, client_id: &Uuid) {
        self.on_disconnect(client_id);
    }
}

fn main() {
    let mut server = Server::create(
        (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
        Box::new(ExampleServer::new()),
    ).expect("Unable to setup server.");
    server.start();
}
