use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;
use monster_spawn::MonsterSpawn;
use entity::Entity;

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

    pub fn get_adjacent_rooms(&self, room_number: u16) -> Option<Vec<&Room>> {
        match self.rooms.get(&room_number) {
            Some(base_room) => {
                let mut result = vec![];

                for room_id in base_room.adjacent_rooms.iter() {
                    result.push(self.get_room(&room_id).unwrap());
                }

                Some(result)
            }
            None => None,
        }
    }

    pub fn get_room_mut(&mut self, room_number: &u16) -> Option<&mut Room> {
        self.rooms.get_mut(room_number)
    }

    pub fn move_player(&mut self, player_id: &Uuid, new_location: u16) -> Result<(), String> {
        if self.has_player(&player_id) && self.has_room(&new_location) {
            {
                let player_room = self.get_player_room_mut(&player_id).unwrap();
                player_room.remove_player(&player_id);
            }
            self.get_room_mut(&new_location)
                .unwrap()
                .place_player(&player_id);
            return Ok(());
        }
        if !self.has_player(player_id) {
            return Err("Player is not in map".to_string());
        }
        if !self.has_room(&new_location) {
            return Err("Target room does not exist.".to_string());
        }
        Err("Move player error default.".to_string())
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

    pub fn complete(self) -> Result<Map, ()> {
        if self.buildee.start_room_id != 0 {
            return Ok(self.buildee);
        }
        Err(())
    }
}
