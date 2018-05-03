extern crate liblurk;
extern crate rand;
extern crate uuid;

mod map;
mod entity;
mod monster_spawn;
mod combat;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use uuid::Uuid;

use liblurk::server::server::{LurkServerError, Server, ServerCallbacks, ServerEventContext};
use liblurk::protocol::protocol_message::*;

use map::{Map, MapBuilder};
use entity::*;
use monster_spawn::monster_spawners;

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
            self.entity_info.get_effective_attack(),
            self.entity_info.get_effective_defense(),
            self.entity_info.get_effective_regen(),
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

        let entry_room_id = map_builder.register_room(
            "Entry Room",
            "This room seems to be the entrance.",
            monster_spawners::null_spawner(),
        );
        let basement_id =
            map_builder.register_room("Basement", "It's very dark, and there seems to be some gross old canned food. It looks like there's a dumbweighter to the attic.", monster_spawners::null_spawner());
        let parlor_id = map_builder.register_room(
            "Parlor",
            "There's a mess of old furniture and music.",
            monster_spawners::null_spawner(),
        );
        let attic_id =
            map_builder.register_room("Attic", "Eek! There's some big spiders up here! There seems to be a dumbweighter to the basement.", monster_spawners::spider_spawner());

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

        context.enqueue_message_this(get_game_packet());
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

    fn on_disconnect(&mut self, client_id: &Uuid) {
        println!("Disconnect made.");
        self.players.remove(client_id);
    }

    fn on_message(
        &mut self,
        context: &mut ServerEventContext,
        message: &Message,
    ) -> LurkServerError {
        println!("Received message packet.");

        if let Some(id) = self.get_player_id_by_name(&message.receiver) {
            context.enqueue_message(message.clone(), id.clone());
        } else {
            println!("On message: bad target.");
            context.enqueue_message_this(
                Error::no_target("Message target does not exist.".to_string()).unwrap(),
            );
        }

        println!("On message completed.");
        return Ok(());
    }

    fn on_change_room(
        &mut self,
        context: &mut ServerEventContext,
        change_room: &ChangeRoom,
    ) -> LurkServerError {
        println!("Change room packet received.");
        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if !player.started {
                context.enqueue_message_this(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                );

                context.enqueue_message_this(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !self.map.has_player(&player.id) {
                context.enqueue_message_this(
                    Error::other("Internal server error: Player not in map.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !self.map.has_room(&change_room.room_number) {
                context.enqueue_message_this(
                    Error::bad_room("Room does not exist.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !self.map
                .get_player_room(&player.id)
                .unwrap()
                .is_adjacent_to(change_room.room_number)
            {
                context.enqueue_message_this(
                    Error::bad_room("Room is not ahead.".to_string()).unwrap(),
                );

                return Ok(());
            }

            match self.map.move_player(&player.id, change_room.room_number) {
                Ok(_) => {
                    player.entity_info.location = change_room.room_number;

                    let player_room = self.map
                        .get_player_room(&player.id)
                        .expect("Bug: Player wasn't moved correctly.");

                    context.enqueue_message_this(
                        Room::new(
                            player_room.get_number(),
                            player_room.get_name(),
                            limit_str_len(&player_room.get_description()),
                        ).expect("Bug: Invalid room packet created."),
                    );

                    for adj_room_num in player_room.get_adjacent_rooms() {
                        let adj_room = self.map
                            .get_room(&adj_room_num)
                            .expect("Bug: Adjacent room doesn't exist.");

                        context.enqueue_message_this(
                            Connection::new(
                                adj_room.get_number(),
                                adj_room.get_name(),
                                adj_room.get_description(),
                            ).unwrap(),
                        );
                    }
                    context.enqueue_message_this(player.get_character_packet());

                    let mut monster_packets = player_room.get_monster_packets();

                    for monster_packet in monster_packets.drain(..) {
                        context.enqueue_message_this(monster_packet);
                    }
                }
                Err(e) => {
                    println!("Move Player Bug: {}", e);
                    return Err(());
                }
            }
        } else {
            context.enqueue_message_this(
                Error::other(
                    "Internal server error: Player not tracked for this session.".to_string(),
                ).unwrap(),
            );
        }
        return Ok(());
    }

    fn on_fight(&mut self, context: &mut ServerEventContext, fight: &Fight) -> LurkServerError {
        println!("Fight packet received.");

        let mut fight_result_message: Option<String> = None;

        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if !player.started {
                context.enqueue_message_this(
                    Error::not_ready("You have not started.".to_string()).unwrap(),
                );
            }

            if let Some(room) = self.map.get_player_room_mut(&context.get_client_id()) {
                if let Some(monster) = room.get_random_monster_mut() {
                    fight_result_message =
                        Some(combat::handle_fight(&mut player.entity_info, monster));
                } else {
                    context.enqueue_message_this(
                        Error::no_target("There are no enemies in this room.".to_string()).unwrap(),
                    );
                }
            } else {
                context.enqueue_message_this(
                    Error::other(
                        "Internal server error: Started player not placed in room.".to_string(),
                    ).unwrap(),
                );
            }
        } else {
            println!("On Fight Error: Untracked player");
        }

        if let Some(message) = fight_result_message {
            if let Some(room) = self.map.get_player_room(&context.get_client_id()) {
                for send_target in room.get_player_ids() {
                    for player_id in room.get_player_ids() {
                        if let Some(player) = self.players.get(&player_id) {
                            context.enqueue_message(
                                player.get_character_packet(),
                                send_target.clone(),
                            );
                        }
                    }
                    for monster in room.get_monster_packets() {
                        context.enqueue_message(monster, send_target.clone());
                    }
                    context.enqueue_message(
                        Message::new(message.clone(), "Server".to_string(), "You".to_string())
                            .unwrap(),
                        send_target.clone(),
                    );
                }
            }
        }

        return Ok(());
    }

    fn on_pvp_fight(
        &mut self,
        context: &mut ServerEventContext,
        pvp_fight: &PvpFight,
    ) -> LurkServerError {
        println!("Pvp fight packet.");
        context.enqueue_message_this(
            Error::no_pvp("Pvp is not currently on this server.".to_string()).unwrap(),
        );
        Ok(())
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, _: &Loot) -> LurkServerError {
        println!("Loot packet received.");
        context.enqueue_message_this(Error::no_target("Cannot loot yet.".to_string()).unwrap());
        Ok(())
    }

    fn on_start(&mut self, context: &mut ServerEventContext, _: &Start) -> LurkServerError {
        println!("Start packet received.");
        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if player.started {
                context.enqueue_message_this(
                    Error::other("You've already started.".to_string()).unwrap(),
                );
                println!("Enqueued you've already started message.");
                return Ok(());
            }

            if player.ready {
                player.started = true;
                player.entity_info.location = self.map.get_start_room().get_number();
                self.map.get_start_room_mut().place_player(&player.id);

                context.enqueue_message_this(player.get_character_packet());
                println!("Enqueued character packet.");

                let player_room = self.map
                    .get_player_room(&player.id)
                    .expect("Bug: Failed to get player room.");

                context.enqueue_message_this(
                    Room::new(
                        player_room.get_number(),
                        player_room.get_name(),
                        player_room.get_description(),
                    ).unwrap(),
                );

                for adj_room_id in player_room.get_adjacent_rooms().iter() {
                    let adj_room = self.map
                        .get_room(&adj_room_id)
                        .expect("Bug: Failed to get adj room.");

                    context.enqueue_message_this(
                        Connection::new(
                            adj_room.get_number(),
                            adj_room.get_name(),
                            adj_room.get_description(),
                        ).unwrap(),
                    );
                }
            } else {
                context.enqueue_message_this(
                    Error::not_ready("You are not ready to start.".to_string()).unwrap(),
                );
            }
        } else {
            context.enqueue_message_this(
                Error::other(
                    "Internal server error: The player for this session is not tracked."
                        .to_string(),
                ).unwrap(),
            );
        }
        Ok(())
    }

    fn on_character(
        &mut self,
        context: &mut ServerEventContext,
        character: &Character,
    ) -> LurkServerError {
        println!("Got character message.");

        if character.attack + character.defense + character.regeneration > INITIAL_POINTS {
            context.enqueue_message_this(
                Error::stat_error("Invalid amount of stat points spent.".to_string()).unwrap(),
            );
            return Ok(());
        }

        if character.attack > STAT_LIMIT || character.defense > STAT_LIMIT
            || character.regeneration > STAT_LIMIT
        {
            context.enqueue_message_this(
                Error::stat_error("One or more attributes were set too high.".to_string()).unwrap(),
            );
            return Ok(());
        }

        if let Some(player) = self.players.get_mut(&context.get_client_id()) {
            if !player.started {
                println!("Accept character!");
                context.enqueue_message_this(Accept::new(CHARACTER_TYPE));
                println!("Accept enqueued!");

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

                context.enqueue_message_this(player.get_character_packet());
            } else {
                context.enqueue_message_this(
                    Error::other("Your stats cannot be edited at this time.".to_string()).unwrap(),
                );
            }
        } else {
            context.enqueue_message_this(
                Error::other(
                    "Internal server error: the player for this session is not tracked."
                        .to_string(),
                ).unwrap(),
            );
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
