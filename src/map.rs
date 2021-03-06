use entity::Entity;
use liblurk::protocol::protocol_message::Character;
use monster_spawn::MonsterSpawn;
use rand::*;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

pub enum LootMonsterResult {
    InvalidTarget,
    MonsterAlive,
    Success(Character),
}

pub enum MovePlayerResult {
    InvalidRoom,
    InvalidPlayer,
    Success,
}

pub struct Map {
    rooms: HashMap<u16, Room>,
    start_room_id: u16,
}

impl Map {
    pub fn get_room(&self, room_number: &u16) -> Option<&Room> {
        self.rooms.get(&room_number)
    }

    pub fn get_start_room(&self) -> &Room {
        self.get_room(&self.start_room_id)
            .expect("Start room does not exist.")
    }

    pub fn get_start_room_mut(&mut self) -> &mut Room {
        let id = self.start_room_id;
        self.get_room_mut(&id).expect("Start room does not exist.")
    }

    pub fn get_room_mut(&mut self, room_number: &u16) -> Option<&mut Room> {
        self.rooms.get_mut(room_number)
    }

    pub fn move_player(&mut self, player_id: &Uuid, new_location: u16) -> MovePlayerResult {
        if !self.has_player(&player_id) {
            return MovePlayerResult::InvalidPlayer;
        }

        if !self.has_room(&new_location) {
            return MovePlayerResult::InvalidRoom;
        }

        {
            let player_room = self.get_player_room_mut(&player_id).unwrap();
            player_room.remove_player(&player_id);
        }

        self.get_room_mut(&new_location)
            .unwrap()
            .place_player(&player_id);
        MovePlayerResult::Success
    }

    pub fn get_player_room_mut(&mut self, player_id: &Uuid) -> Option<&mut Room> {
        for (_, room) in self.rooms.iter_mut() {
            if room.has_player(&player_id) {
                return Some(room);
            }
        }
        None
    }

    pub fn get_player_room(&self, player_id: &Uuid) -> Option<&Room> {
        for (_, room) in self.rooms.iter() {
            if room.has_player(&player_id) {
                return Some(room);
            }
        }
        None
    }

    pub fn has_player(&self, player_id: &Uuid) -> bool {
        for (_, room) in self.rooms.iter() {
            if room.has_player(&player_id) {
                return true;
            }
        }
        false
    }

    pub fn has_room(&self, room_id: &u16) -> bool {
        self.rooms.contains_key(&room_id)
    }

    pub fn clear_player(&mut self, id: &Uuid) {
        for (_, room) in self.rooms.iter_mut() {
            room.remove_player(&id);
        }
    }

    pub fn update_monsters(&mut self) {
        for (_, room) in self.rooms.iter_mut() {
            room.update_monsters();
        }
    }

    pub fn clear_update_flags(&mut self) {
        for (_, room) in self.rooms.iter_mut() {
            room.clear_update_flag();
        }
    }
}

pub struct Room {
    name: String,
    description: String,
    adjacent_rooms: Vec<u16>, // room numbers
    num: u16,
    player_ids: HashSet<Uuid>,
    spawner: Box<MonsterSpawn + Send>,
    monsters: Vec<Entity>,
}

impl Room {
    pub fn get_number(&self) -> u16 {
        self.num
    }

    pub fn place_player(&mut self, player_id: &Uuid) {
        if !self.player_ids.contains(&player_id) {
            self.player_ids.insert(player_id.clone());
        }
    }

    pub fn remove_player(&mut self, player_id: &Uuid) {
        if self.player_ids.contains(&player_id) {
            self.player_ids.remove(&player_id);
        }
    }

    pub fn is_adjacent_to(&self, location: u16) -> bool {
        self.adjacent_rooms.contains(&location)
    }

    pub fn has_player(&self, player_id: &Uuid) -> bool {
        self.player_ids.contains(&player_id)
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_adjacent_rooms(&self) -> &Vec<u16> {
        &self.adjacent_rooms
    }

    pub fn run_spawner(&mut self) {
        self.monsters.extend(self.spawner.spawn_monsters());
    }

    pub fn get_monster_packets(&self, force: bool) -> Vec<Character> {
        let mut result: Vec<Character> = vec![];
        for monster in self.monsters.iter() {
            if monster.update_dirty || force {
                result.push(
                    Character::new(
                        monster.name.clone(),
                        monster.health > 0,
                        true,
                        true,
                        true,
                        true,
                        monster.attack,
                        monster.defense,
                        monster.regen,
                        monster.health,
                        monster.gold,
                        monster.location,
                        monster.desc.clone(),
                    )
                    .expect("Failed on monster to character packet."),
                );
            }
        }
        result
    }

    pub fn get_random_monster_mut(&mut self) -> Option<&mut Entity> {
        if self.monsters.is_empty() || self.all_monsters_dead() {
            return None;
        }

        if self.monsters.len() == 1 {
            return Some(self.monsters.get_mut(0).unwrap());
        }

        let alive_monster_indices = self.get_alive_monsters_indices();

        if alive_monster_indices.len() == 1 {
            let idx = alive_monster_indices[0];
            return self.monsters.get_mut(idx);
        }

        let idx = thread_rng().gen_range(0, alive_monster_indices.len() - 1);

        let monster_idx = alive_monster_indices[idx];
        self.monsters.get_mut(monster_idx)
    }

    pub fn loot_monster(&mut self, target: &String) -> LootMonsterResult {
        if let Some(monster_index) = self.get_monster_index(&target) {
            let is_alive = self.monsters[monster_index].alive;

            if is_alive {
                LootMonsterResult::MonsterAlive
            } else {
                let monster = self.monsters.remove(monster_index);
                LootMonsterResult::Success(
                    Character::new(
                        monster.name.clone(),
                        monster.health > 0,
                        true,
                        true,
                        true,
                        true,
                        monster.attack,
                        monster.defense,
                        monster.regen,
                        monster.health,
                        monster.gold,
                        std::u16::MAX,
                        monster.desc.clone(),
                    )
                    .expect("Failed to create monster packet."),
                )
            }
        } else {
            LootMonsterResult::InvalidTarget
        }
    }

    fn get_monster_index(&self, target: &String) -> Option<usize> {
        for i in 0..self.monsters.len() {
            if self.monsters.get(i).unwrap().name == *target {
                return Some(i);
            }
        }
        None
    }

    fn get_alive_monsters_indices(&self) -> Vec<usize> {
        let mut result = vec![];
        for i in 0..self.monsters.len() {
            if self.monsters.get(i).unwrap().alive {
                result.push(i);
            }
        }
        result
    }

    fn all_monsters_dead(&self) -> bool {
        let mut result = true;
        for monster in self.monsters.iter() {
            if monster.alive {
                result = false;
                break;
            }
        }
        result
    }

    pub fn get_player_ids(&self) -> Vec<Uuid> {
        let mut result = vec![];
        for id in self.player_ids.iter() {
            result.push(id.clone());
        }
        result
    }

    pub fn update_monsters(&mut self) {
        for monster in self.monsters.iter_mut() {
            monster.regen();
        }
    }

    pub fn clear_update_flag(&mut self) {
        for monster in self.monsters.iter_mut() {
            monster.update_dirty = false;
        }
    }
}

pub struct MapBuilder {
    buildee: Map,
    room_number: u16,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder {
            buildee: Map {
                rooms: HashMap::new(),
                start_room_id: 0,
            },
            room_number: 1,
        }
    }

    pub fn register_room<T: Into<String>, S: Into<String>>(
        &mut self,
        name: T,
        description: S,
        monster_spawner: Box<MonsterSpawn + Send>,
    ) -> u16 {
        let room = Room {
            name: name.into(),
            description: description.into(),
            adjacent_rooms: vec![],
            num: self.room_number,
            player_ids: HashSet::new(),
            spawner: monster_spawner,
            monsters: vec![],
        };

        self.buildee.rooms.insert(self.room_number, room);
        self.room_number += 1;

        self.room_number - 1
    }

    pub fn link_rooms(&mut self, room1_id: u16, room2_id: u16) -> Result<(), ()> {
        if !self.buildee.rooms.contains_key(&room1_id)
            || !self.buildee.rooms.contains_key(&room2_id)
        {
            return Err(());
        }

        {
            let room1 = self.buildee.get_room_mut(&room1_id).unwrap();
            room1.adjacent_rooms.push(room2_id);
        }

        {
            let room2 = self.buildee.get_room_mut(&room2_id).unwrap();
            room2.adjacent_rooms.push(room1_id);
        }

        Ok(())
    }

    pub fn set_start_room(&mut self, room_num: u16) -> Result<(), ()> {
        if !self.buildee.rooms.contains_key(&room_num) {
            return Err(());
        }

        self.buildee.start_room_id = room_num;

        Ok(())
    }

    pub fn complete(mut self) -> Result<Map, ()> {
        if self.buildee.start_room_id != 0 {
            for (_, room) in self.buildee.rooms.iter_mut() {
                room.run_spawner();
            }

            return Ok(self.buildee);
        }
        Err(())
    }
}
