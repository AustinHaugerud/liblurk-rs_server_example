use std::path::Path;
use game::types::Location;
use std::error::Error;
use ron::de::from_reader;
use specs::{World, Entity};
use std::collections::HashMap;
use specs::world::Builder;
use game::components::location::ConnectedLocations;

pub struct LocationLoader {
    load_dir: String,
}

impl LocationLoader {
    pub fn new(path: &str) -> LocationLoader {
        LocationLoader {
            load_dir: path.to_string()
        }
    }

    pub fn load_location_definitions(&self) -> Result<Vec<Location>, String> {
        use std::fs;

        let mut locations = vec![];

        let paths = fs::read_dir(&self.load_dir)
            .map_err(|e| format!("Failed to open locations directory: {}", e.description()))?;

        for rentry in paths {
            let entry =
                rentry.map_err(|e| format!("Erroneous directory entry: {}", e.description()))?;
            let path = entry.path();

            if path.is_file() {
                let file = fs::File::open(&path)
                    .map_err(|e| format!("Failed to open location definition file: {}", e.description()))?;

                let location: Location = from_reader(file).map_err(|e| format!("Failed to parse location definition {:?}: {:?}", &path, e))?;

                locations.push(location);
            }
        }

        Ok(locations)
    }
}

pub fn add_locations_to_world(loader: LocationLoader, world: &mut World, start_location_name: &str) -> Result<Entity, String> {

    use game::components::location;
    use specs::prelude::*;

   let mut locations = loader.load_location_definitions()?;

    if locations.len() > std::u16::MAX as usize {
        return Err("Too many locations declared in locations directory.".to_string());
    }

    let mut location_map = {
        let mut mapping = HashMap::new();

        for location in locations.drain(..) {
            if mapping.contains_key(&location.name) {
                return Err(format!("Location name {} not unique.", location.name));
            }
            else {
                mapping.insert(location.name.clone(), location);
            }
        }

        mapping
    };

    // Check that no connections are invalid
    for (_, location) in location_map.iter() {
        for connection in location.connections.iter() {
            if !location_map.contains_key(connection) {
                return Err(format!("Connection {} declared in location {} invalid.", connection, &location.name));
            }
        }
    }

    let mut room_num = 1u16;

    let mut entities = HashMap::new();

    // Register locations without connections
    for (name, location) in location_map.iter() {
        let entity = world.create_entity()
            .with(location::Number(room_num))
            .with(location::Name(name.clone()))
            .with(location::Description(location.description.clone()))
            .with(location::ContainedEntities(vec![]))
            .with(location::Factions(location.factions.clone()))
            .with(location::Stones(location.stones.clone()))
            .with(location::ConnectedLocations(vec![]))
            .build();
        room_num += 1;
        entities.insert(name.clone(), entity);
    }

    let start_location = *entities.get(start_location_name).ok_or(format!("Invalid start location {}.", start_location_name))?;

    let mut connection_storage = world.write_storage::<ConnectedLocations>();

    for (name, location) in location_map.drain() {
        let loc = *entities.get(&name).unwrap();
        let mut connections = connection_storage.get_mut(loc).unwrap();
        for connection in location.connections {
            let conn_entity = *entities.get(&connection).unwrap();
            connections.0.push(conn_entity);
        }
    }

    Ok(start_location)
}
