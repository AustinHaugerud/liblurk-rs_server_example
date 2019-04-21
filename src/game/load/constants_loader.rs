use game::types::GameConstants;
use ron::de::from_reader;

pub struct ConstantsLoader {
    path : String,
}

impl ConstantsLoader {
    pub fn new(path: &str) -> ConstantsLoader {
        ConstantsLoader {
            path: path.to_string()
        }
    }

    pub fn load_constants(&self) -> Result<GameConstants, String> {
        use std::fs;
        let file = fs::File::open(&self.path).map_err(|e| format!("Failed to open game constants file: {:?}", e))?;

        let constants: GameConstants = from_reader(file)
            .map_err(|e| format!("Failed to parse game constants: {:?}", e))?;

        Ok(constants)
    }
}
