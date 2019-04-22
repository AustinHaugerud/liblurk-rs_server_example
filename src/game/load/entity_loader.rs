use game::types::NPCEntity;
use ron::de::from_reader;
use std::error::Error;
use std::path::Path;

pub struct EntityLoader {
    load_dir: String,
}

impl EntityLoader {
    pub fn new(path: &str) -> EntityLoader {
        EntityLoader {
            load_dir: path.to_string(),
        }
    }

    pub fn load_entity_definitions(&self) -> Result<Vec<NPCEntity>, String> {
        use std::fs;

        let mut entities = vec![];

        let paths = fs::read_dir(&self.load_dir)
            .map_err(|e| format!("Failed to open entities directory: {}", e.description()))?;

        for rentry in paths {
            let entry =
                rentry.map_err(|e| format!("Erroneous directory entry: {}", e.description()))?;
            let path = entry.path();

            if path.is_file() {
                let file = fs::File::open(path)
                    .map_err(|e| format!("Failed to open file: {}", e.description()))?;

                let entity: NPCEntity = from_reader(file).map_err(|e| {
                    format!("Failed to parse entity definition: {}", e.description())
                })?;

                entities.push(entity);
            }
        }

        Ok(entities)
    }
}
