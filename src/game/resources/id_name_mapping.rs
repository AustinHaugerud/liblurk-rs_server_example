use std::collections::HashMap;
use uuid::Uuid;

/// A bijective mapping between unique ids and names.
#[derive(Default)]
pub struct IdNameMapping {
    id_to_name: HashMap<Uuid, String>,
    name_to_id: HashMap<String, Uuid>,
}

impl IdNameMapping {
    pub fn get_name(&self, id: &Uuid) -> Option<&String> {
        self.id_to_name.get(id)
    }

    pub fn get_id(&self, name: &str) -> Option<&Uuid> {
        self.name_to_id.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.name_to_id.contains_key(name)
    }

    pub fn insert(&mut self, id: &Uuid, name: &str) {
        if !self.contains(name) {
            // Remove Name -> Id
            if let Some(old_name) = self.id_to_name.get(id) {
                self.name_to_id.remove(old_name);
            }

            // Remove Id -> Name
            self.id_to_name.remove(id);

            self.name_to_id.insert(name.to_string(), *id);
            self.id_to_name.insert(*id, name.to_string());
        }
    }
}
