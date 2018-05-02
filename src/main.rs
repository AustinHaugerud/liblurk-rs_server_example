extern crate liblurk;
extern crate rand;
extern crate uuid;

mod map;
mod entity;
mod monster_spawn;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use uuid::Uuid;

use liblurk::server::server::{Server, ServerCallbacks, ServerEventContext, LurkServerError};
use liblurk::protocol::protocol_message::*;

use map::{Map, MapBuilder};
use entity::*;

const INITIAL_POINTS: u16 = 100;
const STAT_LIMIT: u16 = 100;

const DEFAULT_HEALTH: i16 = 500;
const DEFAULT_GOLD: u16 = 0;

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
    id: Uuid,
}

fn limit_str_len(string: &String) -> String {
    if string.len() >= u16::max_value() as usize {
        return string.chars().skip(u16::max_value() as usize).collect();
    }

    string.clone()
}

impl Player {
    fn get_character_packet(&self) -> Character {
        Character::new(
            limit_str_len(&self.entity_info.name),
            self.entity_info.alive,
            true,
            false,
            self.started,
            self.ready,
            self.entity_info.attack,
            self.entity_info.defense,
            self.entity_info.regen,
            self.entity_info.health,
            self.entity_info.gold,
            self.entity_info.location,
            limit_str_len(&self.entity_info.desc),
        ).expect("Invalid character packet from player instance.")
    }
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

        map_builder
            .set_start_room(entry_room_id)
            .expect("Failed to set starting room.");

        let map = map_builder.complete().expect("Failed to build map");

        ExampleServer {
            players: HashMap::new(),
            map,
        }
    }

    fn get_player_id_by_name(&self, search_name: &String) -> Option<Uuid> {
        for (id, player) in self.players.iter() {
            if search_name.eq(&player.entity_info.name) {
                return Some(id.clone());
            }
        }
        None
    }
}

impl ServerCallbacks for ExampleServer {
    fn on_connect(&mut self, context: &mut ServerEventContext) -> LurkServerError {
        println!("Connection made!");
        match context.get_send_channel().write_message(get_game_packet()) {
            Ok(()) => {
                self.players.insert(
                    context.get_client_id(),
                    Player {
                        entity_info: Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        },
                        ready: false,
                        started: false,
                        id: context.get_client_id().clone(),
                    },
                );
                Ok(())
            }
            Err(_) => Err(())
        }
    }

    fn on_disconnect(&mut self, client_id: &Uuid) {
        println!("Disconnect made.");
        self.players.remove(client_id);
    }

    fn on_message(&mut self, context: &mut ServerEventContext, message: &Message) -> LurkServerError {
        println!("Received message packet.");

        {
            if !self.players.contains_key(&context.get_client_id()) {
                println!("Error: player not tracked.");
                return Err(());
            }
        }

        // If the client sends a message to themselves, we must handle it as a special case. If the logic farther below
        // handles it, the lock will not be acquirable, and we'll deadlock.
        {
            println!("Player messaged themselves.");
            let player_name = self.players
                .get(&context.get_client_id())
                .unwrap()
                .entity_info
                .name
                .clone();

            if message.receiver == player_name {
                context.get_send_channel().write_message_ref(message)?
            }

            return Ok(());
        }

        if let Some(id) = self.get_player_id_by_name(&message.receiver) {
            println!("Player messaged other player.");
            match context.get_client(&id) {
                Ok(op) => match op {
                    Some(m) => match m.lock() {
                        Ok(c) => { context.get_send_channel().write_message_ref(message)?; },
                        Err(_) => { println!("On message: poison error."); return Err(()); },
                    },
                    None => {
                        println!("Did not retrieve client as expected.");
                    }
                },
                Err(_) => {
                    println!("On message: Error getting client.");
                    return Err(());
                },
            }
        } else {
            context.get_send_channel().write_message(
                Error::no_target("Message target does not exist.".to_string()).unwrap(),
            )?;
        }

        println!("On message completed.");
        return Ok(());
    }

    fn on_change_room(&mut self, context: &mut ServerEventContext, change_room: &ChangeRoom) -> LurkServerError {
        println!("Change room packet received.");
        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if !player.started {
                context.get_send_channel().write_message(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                )?;

                context.get_send_channel().write_message(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                )?;

                return Ok(());
            }

            if !self.map.has_player(&player.id) {
                context
                    .get_send_channel()
                    .write_message(
                        Error::other("Internal server error: Player not in map.".to_string())
                            .unwrap(),
                    )?;

                return Ok(());
            }

            if !self.map.has_room(&change_room.room_number) {
                context
                    .get_send_channel()
                    .write_message(Error::bad_room("Room does not exist.".to_string()).unwrap())?;

                return Ok(());
            }

            if !self.map
                .get_player_room(&player.id)
                .unwrap()
                .is_adjacent_to(change_room.room_number)
            {
                context
                    .get_send_channel()
                    .write_message(Error::bad_room("Room is not ahead.".to_string()).unwrap())?;

                return Ok(());
            }

            match self.map.move_player(&player.id, change_room.room_number) {
                Ok(_) => {
                    player.entity_info.location = change_room.room_number;

                    let player_room = self.map
                        .get_player_room(&player.id)
                        .expect("Bug: Player wasn't moved correctly.");

                    context
                        .get_send_channel()
                        .write_message(
                            Room::new(
                                player_room.get_number(),
                                player_room.get_name(),
                                limit_str_len(&player_room.get_description()),
                            ).expect("Bug: Invalid room packet created."),
                        )?;

                    for adj_room_num in player_room.get_adjacent_rooms() {
                        let adj_room = self.map
                            .get_room(&adj_room_num)
                            .expect("Bug: Adjacent room doesn't exist.");

                        context
                            .get_send_channel()
                            .write_message(
                                Connection::new(
                                    adj_room.get_number(),
                                    adj_room.get_name(),
                                    adj_room.get_description(),
                                ).unwrap(),
                            )?;
                    }
                }
                Err(e) => { println!("Move Player Bug: {}", e); return Err(()); },
            }
        } else {
            context
                .get_send_channel()
                .write_message(
                    Error::other(
                        "Internal server error: Player not tracked for this session.".to_string(),
                    ).unwrap(),
                )?;
        }
        return Ok(());
    }

    fn on_fight(&mut self, context: &mut ServerEventContext, fight: &Fight) -> LurkServerError {
        println!("Fight packet received.");
        context
            .get_send_channel()
            .write_message(
                Error::no_fight("Server does not have fighting yet.".to_string()).unwrap(),
            )
    }

    fn on_pvp_fight(&mut self, context: &mut ServerEventContext, pvp_fight: &PvpFight) -> LurkServerError {
        println!("Pvp fight packet.");
        context
            .get_send_channel()
            .write_message(
                Error::no_pvp("Pvp is not currently on this server.".to_string()).unwrap(),
            )
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, _: &Loot) -> LurkServerError {
        println!("Loot packet received.");
        context
            .get_send_channel()
            .write_message(Error::no_target("Cannot loot yet.".to_string()).unwrap())
    }

    fn on_start(&mut self, context: &mut ServerEventContext, _: &Start) -> LurkServerError {
        println!("Start packet received.");
        if let Some(player) = self.players.get_mut(&context.get_client_id()) {

            if player.started {
                context.get_send_channel().write_message(Error::other("You've already started.".to_string()).unwrap())?;
                return Ok(());
            }

            if player.ready {
                player.started = true;
                player.entity_info.location = self.map.get_start_room().get_number();
                self.map.get_start_room_mut().place_player(&player.id);

                context
                    .get_send_channel()
                    .write_message(player.get_character_packet())?;


                let player_room = self.map
                    .get_player_room(&player.id)
                    .expect("Bug: Failed to get player room.");

                context
                    .get_send_channel()
                    .write_message(
                        Room::new(
                            player_room.get_number(),
                            player_room.get_name(),
                            player_room.get_description(),
                        ).unwrap(),
                    )?;


                for adj_room_id in player_room.get_adjacent_rooms().iter() {
                    let adj_room = self.map
                        .get_room(&adj_room_id)
                        .expect("Bug: Failed to get adj room.");

                    context
                        .get_send_channel()
                        .write_message(
                            Connection::new(
                                adj_room.get_number(),
                                adj_room.get_name(),
                                adj_room.get_description(),
                            ).unwrap(),
                        )?;
                }
            } else {
                context
                    .get_send_channel()
                    .write_message(
                        Error::not_ready("You are not ready to start.".to_string()).unwrap(),
                    )?;
            }
        } else {
            context
                .get_send_channel()
                .write_message(
                    Error::other(
                        "Internal server error: The player for this session is not tracked."
                            .to_string(),
                    ).unwrap(),
                )?;
        }
        Ok(())
    }

    fn on_character(&mut self, context: &mut ServerEventContext, character: &Character) -> LurkServerError {
        println!("Got character message.");

        if character.attack + character.defense + character.regeneration > INITIAL_POINTS {
            return context
                .get_send_channel()
                .write_message(
                    Error::stat_error("Invalid amount of stat points spent.".to_string()).unwrap(),
                );
        }

        if character.attack > STAT_LIMIT || character.defense > STAT_LIMIT
            || character.regeneration > STAT_LIMIT
        {
            return context
                .get_send_channel()
                .write_message(
                    Error::stat_error("One or more attributes were set too high.".to_string())
                        .unwrap(),
                );

        }

        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if !player.started {
                context
                    .get_send_channel()
                    .write_message(Accept::new(CHARACTER_TYPE))?;

                player.ready = true;
                player.entity_info = Entity {
                    name: character.player_name.clone(),
                    attack: character.attack,
                    defense: character.defense,
                    regen: character.regeneration,
                    health: DEFAULT_HEALTH,
                    gold: DEFAULT_GOLD,
                    location: 0,
                    alive: true,
                    monster: false,
                    desc: character.description.clone(),
                };
            } else {
                context
                    .get_send_channel()
                    .write_message(
                        Error::other("Your stats cannot be edited at this time.".to_string())
                            .unwrap(),
                    )?;
            }
        } else {
            context
                .get_send_channel()
                .write_message(
                    Error::other(
                        "Internal server error: the player for this session is not tracked."
                            .to_string(),
                    ).unwrap(),
                )?;
        }
        Ok(())
    }

    fn on_leave(&mut self, client_id: &Uuid) -> LurkServerError {
        println!("Leave packet received.");
        self.on_disconnect(client_id);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port_number = args.get(1)
        .expect("Insufficient arguments")
        .parse::<u16>()
        .expect("Failed to parse port number.");

    let mut server = Server::create(
        (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_number),
        Box::new(ExampleServer::new()),
    ).expect("Unable to create server.");
    match server.start() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Failed to start server"),
    };
}
