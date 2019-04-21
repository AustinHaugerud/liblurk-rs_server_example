use std::path::Path;
use game::types::Location;
use std::error::Error;
use ron::de::from_reader;

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
