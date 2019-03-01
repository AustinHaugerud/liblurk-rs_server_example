extern crate liblurk;
extern crate rand;
extern crate uuid;
#[macro_use] extern crate nickel;

mod map;
mod entity;
mod monster_spawn;
mod combat;
mod rest;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

use uuid::Uuid;

use liblurk::server::server::{LurkServerError, Server, ServerCallbacks, ServerEventContext,
                              UpdateContext};
use liblurk::protocol::protocol_message::*;

use map::{Map, MapBuilder};
use entity::*;
use monster_spawn::monster_spawners;
use monster_spawn::monster_spawners::MolePeopleLevel;
use map::LootMonsterResult;
use map::MovePlayerResult;

use std::sync::Arc;
use std::sync::Mutex;
use rest::RestService;
use std::thread;

const INITIAL_POINTS: u16 = 600;
const STAT_LIMIT: u16 = u16::max_value();

const DEFAULT_HEALTH: i16 = 500;
const DEFAULT_GOLD: u16 = 0;

pub fn get_game_packet() -> Game {
    Game {
        initial_points: INITIAL_POINTS,
        stat_limit: STAT_LIMIT,
        description: String::from("You find yourself in an uneventful boring dungeon."),
    }
}

pub struct Player {
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
    players: Arc<Mutex<HashMap<Uuid, Player>>>,
    map: Arc<Mutex<Map>>,
    last_update_time: Instant,
}

impl ExampleServer {
    fn new() -> ExampleServer {
        let mut map_builder = MapBuilder::new();

        let entry_room_id = map_builder.register_room(
            "Entry Room",
            "This room seems to be the entrance.",
            monster_spawners::mean_butler_spawner(),
        );
        let basement_id =
            map_builder.register_room("Basement", "It's very dark, and there seems to be some gross old canned food. It looks like there's a dumbweighter to the attic.", monster_spawners::derry_spawner());
        let parlor_id = map_builder.register_room(
            "Parlor",
            "There's a mess of old furniture and music.",
            monster_spawners::creepy_uncle_spawner(),
        );
        let attic_id =
            map_builder.register_room("Attic", "Eek! There's some big spiders up here! There seems to be a dumbweighter to the basement.", monster_spawners::spider_spawner());

        let badger_den = map_builder.register_room(
            "Badger Den",
            "A honey badger seems to taken refuge here",
            monster_spawners::honey_badger_spawner(),
        );
        let cavern_hall = map_builder.register_room(
            "Cavern Hall",
            "A hallway piece of the cavern",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Low, (3, 5)),
        );
        let barracks_north = map_builder.register_room(
            "Mole Barracks North",
            "The north wing of the mole barracks.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Low, (3, 8)),
        );
        let barracks_east = map_builder.register_room(
            "Mole Barracks East",
            "The east wing of the mole barracks.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Low, (4, 10)),
        );
        let barracks_west = map_builder.register_room(
            "Mole Barracks West",
            "The west wing of the mole barracks.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Low, (5, 10)),
        );
        let pit = map_builder.register_room(
            "Pit",
            "A dark pit filled with spiders.",
            monster_spawners::spider_spawner(),
        );
        let nursery = map_builder.register_room(
            "Mole Nursery",
            "A disgusting nursery cavern filled with mole people.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Low, (15, 20)),
        );
        let cache = map_builder.register_room(
            "Cache",
            "A cavern serving as a food cache.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Mid, (5, 8)),
        );
        let cavern = map_builder.register_room(
            "Cavern",
            "A cavern filled with mole people.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Mid, (15, 20)),
        );
        let deep_cavern = map_builder.register_room(
            "Deep Cavern",
            "Another deeper cavern filled with yet more mole people.",
            monster_spawners::composite_spawner(vec![
                monster_spawners::mole_people_spawner(MolePeopleLevel::Mid, (15, 20)),
                monster_spawners::mole_high_priest_spawner(),
            ]),
        );
        let mole_grounds = map_builder.register_room(
            "Mole Grounds",
            "A large living area of the mole people.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (15, 20)),
        );
        let spawn_grounds = map_builder.register_room(
            "Spawn Grounds",
            "A spawning grounds where the queens spawn more moles.",
            monster_spawners::composite_spawner(vec![
                monster_spawners::mole_queen_spawner(),
                monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (25, 30)),
            ]),
        );
        let mole_pit_west = map_builder.register_room(
            "Mole Pit West",
            "The west mole pit.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (10, 12)),
        );
        let mole_pit_east = map_builder.register_room(
            "Mole Pit East",
            "The east mole pit.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (10, 12)),
        );
        let mole_pit_south = map_builder.register_room(
            "Mole Pit South",
            "The south mole pit.",
            monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (10, 12)),
        );
        let temple = map_builder.register_room(
            "Mole Temple",
            "A cavern temple it seems.",
            monster_spawners::composite_spawner(vec![
                monster_spawners::mole_high_priest_spawner(),
                monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (30, 35)),
            ]),
        );
        let goliath_gate = map_builder.register_room(
            "Goliath Gate",
            "The goliath guards here!",
            monster_spawners::composite_spawner(vec![
                monster_spawners::great_mole_goliath_spawner(),
                monster_spawners::mole_people_spawner(MolePeopleLevel::Hard, (5, 10)),
            ]),
        );
        let pit_of_queens = map_builder.register_room(
            "Pit of Queens",
            "The queens rule from here.",
            monster_spawners::composite_spawner(vec![
                monster_spawners::pit_of_queens_spawner(),
                monster_spawners::mole_people_spawner(MolePeopleLevel::Mid, (40, 50)),
            ]),
        );
        let cavern_end = map_builder.register_room(
            "Cavern End",
            "This is the end, the smell is awful.",
            monster_spawners::homonculus_spawner(),
        );

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

        map_builder.link_rooms(basement_id, badger_den).unwrap();
        map_builder.link_rooms(badger_den, cavern_hall).unwrap();
        map_builder.link_rooms(badger_den, barracks_north).unwrap();
        map_builder.link_rooms(badger_den, cavern).unwrap();
        map_builder
            .link_rooms(barracks_north, barracks_east)
            .unwrap();
        map_builder
            .link_rooms(barracks_north, barracks_west)
            .unwrap();
        map_builder
            .link_rooms(barracks_east, barracks_west)
            .unwrap();
        map_builder.link_rooms(cavern_hall, pit).unwrap();
        map_builder.link_rooms(cavern_hall, nursery).unwrap();
        map_builder.link_rooms(cavern_hall, cache).unwrap();
        map_builder.link_rooms(cavern, deep_cavern).unwrap();
        map_builder.link_rooms(deep_cavern, mole_grounds).unwrap();
        map_builder.link_rooms(mole_grounds, spawn_grounds).unwrap();
        map_builder.link_rooms(mole_grounds, mole_pit_west).unwrap();
        map_builder.link_rooms(mole_grounds, mole_pit_east).unwrap();
        map_builder
            .link_rooms(mole_grounds, mole_pit_south)
            .unwrap();
        map_builder.link_rooms(mole_grounds, temple).unwrap();
        map_builder.link_rooms(temple, goliath_gate).unwrap();
        map_builder.link_rooms(goliath_gate, pit_of_queens).unwrap();
        map_builder.link_rooms(pit_of_queens, cavern_end).unwrap();

        map_builder
            .set_start_room(entry_room_id)
            .expect("Failed to set starting room.");

        let map = map_builder.complete().expect("Failed to build map");

        ExampleServer {
            players: Arc::new(Mutex::new(HashMap::new())),
            map : Arc::new(Mutex::new(map)),
            last_update_time: Instant::now(),
        }
    }

    fn get_player_id_by_name(&self, search_name: &String) -> Option<Uuid> {
        for (id, player) in self.players.lock().unwrap().iter() {
            if search_name.eq(&player.entity_info.name) {
                return Some(id.clone());
            }
        }
        None
    }

    pub fn map(&self) -> Arc<Mutex<Map>> {
        self.map.clone()
    }

    pub fn players(&self) -> Arc<Mutex<HashMap<Uuid, Player>>> {
        self.players.clone()
    }
}

impl ServerCallbacks for ExampleServer {
    fn on_connect(&mut self, context: &mut ServerEventContext) -> LurkServerError {
        println!("Connection made!");

        context.enqueue_message_this(get_game_packet());
        self.players.lock().unwrap().insert(
            context.get_client_id(),
            Player {
                entity_info: Entity {
                    update_dirty: false,
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
                    base_health: DEFAULT_HEALTH,
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
        self.players.lock().unwrap().remove(client_id);
        self.map.lock().unwrap().clear_player(&client_id);
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
        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(&context.get_client_id()) {
            if !player.started {
                context.enqueue_message_this(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                );

                context.enqueue_message_this(
                    Error::not_ready("You have not started yet.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !player.entity_info.alive {
                context.enqueue_message_this(Error::other("The dead cannot move.".to_string()).unwrap());
                return Ok(());
            }

            if !self.map.lock().unwrap().has_player(&player.id) {
                context.enqueue_message_this(
                    Error::other("Internal server error: Player not in map.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !self.map.lock().unwrap().has_room(&change_room.room_number) {
                context.enqueue_message_this(
                    Error::bad_room("Room does not exist.".to_string()).unwrap(),
                );

                return Ok(());
            }

            if !self.map.lock().unwrap()
                .get_player_room(&player.id)
                .unwrap()
                .is_adjacent_to(change_room.room_number)
            {
                context.enqueue_message_this(
                    Error::bad_room("Room is not ahead.".to_string()).unwrap(),
                );

                return Ok(());
            }

            let old_room_id = player.entity_info.location;

            match self.map.lock().unwrap().move_player(&player.id, change_room.room_number) {
                MovePlayerResult::InvalidRoom => {
                    context.enqueue_message_this(Error::no_target("Invalid room.".to_string()).unwrap());
                },
                MovePlayerResult::InvalidPlayer => {
                    println!("Move player bug: Player not recognized.");
                    return Err(());
                },
                MovePlayerResult::Success => {

                    let map = self.map.lock().unwrap();
                    let old_room = map.get_room(&old_room_id).expect("Old room not found.");

                    player.entity_info.location = change_room.room_number;

                    for player_id in old_room.get_player_ids() {
                        context.enqueue_message(player.get_character_packet(), player_id);
                    }

                    let player_room = map
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
                        let adj_room = map
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

                    let mut monster_packets = player_room.get_monster_packets(true);

                    for monster_packet in monster_packets.drain(..) {
                        context.enqueue_message_this(monster_packet);
                    }
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

    fn on_fight(&mut self, context: &mut ServerEventContext, _: &Fight) -> LurkServerError {
        println!("Fight packet received.");

        let mut fight_result_message: Option<String> = None;

        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(&context.get_client_id()) {
            if !player.started {
                context.enqueue_message_this(
                    Error::not_ready("You have not started.".to_string()).unwrap(),
                );
                return Ok(());
            }

            if !player.entity_info.alive {
                context.enqueue_message_this(Error::other("The dead cannot fight.".to_string()).unwrap());
                return Ok(());
            }

            if let Some(room) = self.map.lock().unwrap().get_player_room_mut(&context.get_client_id()) {
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
            if let Some(room) = self.map.lock().unwrap().get_player_room(&context.get_client_id()) {
                for send_target in room.get_player_ids() {
                    for player_id in room.get_player_ids() {
                        if let Some(player) = players.get(&player_id) {
                            context.enqueue_message(
                                player.get_character_packet(),
                                send_target.clone(),
                            );
                        }
                    }
                    for monster in room.get_monster_packets(false) {
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

    fn on_pvp_fight(&mut self, context: &mut ServerEventContext, _: &PvpFight) -> LurkServerError {
        println!("Pvp fight packet.");
        context.enqueue_message_this(
            Error::no_pvp("Pvp is not currently on this server.".to_string()).unwrap(),
        );
        Ok(())
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, loot: &Loot) -> LurkServerError {
        println!("Loot packet received.");

        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(&context.get_client_id()) {

            if !player.entity_info.alive {
                context.enqueue_message_this(Error::other("You cannot loot when you are dead.".to_string()).unwrap());
                return Ok(());
            }

            if !player.started {
                context.enqueue_message_this(Error::not_ready("You have not started.".to_string()).unwrap());
                return Ok(());
            }

            if let Some(room) = self.map.lock().unwrap().get_player_room_mut(&context.get_client_id()) {
                match room.loot_monster(&loot.target) {
                    LootMonsterResult::InvalidTarget => {
                        context.enqueue_message_this(Error::no_target("Invalid target.".to_string()).unwrap());
                    },
                    LootMonsterResult::MonsterAlive => {
                        context.enqueue_message_this(Error::no_target("Can't loot living target.".to_string()).unwrap());
                    },
                    LootMonsterResult::Success(mut monster) => {
                        player.entity_info.gold += monster.gold;
                        monster.gold = 0;
                        player.entity_info.update_dirty = true;

                        for player_id in room.get_player_ids() {
                            println!("Notifying player {:?} of monster removal.", &player_id);
                            context.enqueue_message(monster.clone(), player_id.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn on_start(&mut self, context: &mut ServerEventContext, _: &Start) -> LurkServerError {
        println!("Start packet received.");
        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(&context.get_client_id()) {
            if player.started {
                context.enqueue_message_this(
                    Error::other("You've already started.".to_string()).unwrap(),
                );
                println!("Enqueued you've already started message.");
                return Ok(());
            }

            if player.ready {
                player.started = true;
                player.entity_info.location = self.map.lock().unwrap().get_start_room().get_number();
                self.map.lock().unwrap().get_start_room_mut().place_player(&player.id);

                context.enqueue_message_this(player.get_character_packet());
                println!("Enqueued character packet.");

                let map = self.map.lock().unwrap();
                let player_room = map
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
                    let adj_room = map
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

        let players = self.players.lock().unwrap();
        if let Some(player) = players.get(&context.get_client_id()) {
            if player.started {
                if let Some(player_room) = self.map.lock().unwrap().get_player_room(&context.get_client_id()) {
                    for player_id in player_room.get_player_ids() {
                        if let Some(player) = players.get(&player_id) {
                            context.enqueue_message_this(player.get_character_packet());
                        }
                        context.enqueue_message(player.get_character_packet(), player_id.clone());
                    }
                    for monster in player_room.get_monster_packets(true) {
                        context.enqueue_message_this(monster);
                    }
                }
            }
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

        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(&context.get_client_id()) {
            if !player.started {
                println!("Accept character!");
                context.enqueue_message_this(Accept::new(CHARACTER_TYPE));
                println!("Accept enqueued!");

                player.ready = true;

                let attack_boost =
                    (1.25f32 * (character.attack as f32 / INITIAL_POINTS as f32)).max(1f32);
                let defense_boost =
                    (1.25f32 * (character.defense as f32 / INITIAL_POINTS as f32)).max(1f32);
                let regen_boost =
                    (1.25f32 * (character.regeneration as f32 / INITIAL_POINTS as f32)).max(1f32);

                let attack = (character.attack as f32 * attack_boost).floor() as u16;
                let defense = (character.defense as f32 * defense_boost).floor() as u16;
                let regen = (character.regeneration as f32 * regen_boost).floor() as u16;

                player.entity_info = Entity {
                    update_dirty: false,
                    name: character.player_name.clone(),
                    attack,
                    defense,
                    regen,
                    health: DEFAULT_HEALTH,
                    gold: DEFAULT_GOLD,
                    location: 0,
                    alive: true,
                    monster: false,
                    desc: character.description.clone(),
                    base_health: DEFAULT_HEALTH,
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

    fn update(&mut self, context: &UpdateContext) {
        let current = Instant::now();
        if current.duration_since(self.last_update_time) > Duration::from_secs(1) {
            println!("Update: {:?}", current);
            self.last_update_time = current;

            let mut players = self.players.lock().unwrap();

            for (_, player) in players.iter_mut() {
                player.entity_info.regen();
            }

            self.map.lock().unwrap().update_monsters();

            for (target_id, _) in players.iter() {
                if let Some(player_room) = self.map.lock().unwrap().get_player_room(&target_id) {
                    for player_id in player_room.get_player_ids() {
                        if let Some(player) = players.get(&player_id) {
                            if player.entity_info.update_dirty {
                                context.enqueue_message(
                                    player.get_character_packet(),
                                    target_id.clone(),
                                );
                            }
                        }
                    }

                    for monster in player_room.get_monster_packets(false) {
                        context.enqueue_message(monster, target_id.clone());
                    }
                }
            }

            self.map.lock().unwrap().clear_update_flags();

            for (_, player) in players.iter_mut() {
                player.entity_info.update_dirty = false;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port_number = args.get(1)
        .expect("Insufficient arguments")
        .parse::<u16>()
        .expect("Failed to parse port number.");

    let behaviour = ExampleServer::new();

    let rest_server = RestService::new(behaviour.map(), behaviour.players()).expect("Failed to create REST service.");

    let mut server = Server::create(
        (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port_number),
        Box::new(behaviour),
    ).expect("Unable to create server.");
    thread::spawn(move || {
        rest_server.start(port_number + 1);
    });
    match server.start() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Failed to start server"),
    };
}
