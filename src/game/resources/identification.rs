use std::collections::HashMap;
use uuid::Uuid;
use specs::Entity;

pub struct Identification {
    name_to_uid : HashMap<String, Uuid>,
    name_to_entity : HashMap<String, Entity>,

    uid_to_entity : HashMap<Uuid, Entity>,
    entity_to_uid : HashMap<Entity, Uuid>
}

impl Identification {

    pub fn new() -> Identification {
        Identification {
            name_to_uid: HashMap::new(),
            name_to_entity: HashMap::new(),
            uid_to_entity: HashMap::new(),
            entity_to_uid: HashMap::new()
        }
    }

    pub fn register(&mut self, uid : &Uuid, name : &str, entity : Entity) {
        self.name_to_uid.insert(name.clone(), uid.clone());
        self.name_to_entity.insert(name.clone(), entity);
        self.uid_to_entity.insert(uid.clone(), entity);
        self.entity_to_uid.insert(entity, uid.clone());
    }

    pub fn uid_from_name(&self,  name : &str) -> Option<&Uuid> {
        self.name_to_uid.get(name)
    }

    pub fn entity_from_name(&self, name : &str) -> Option<&Entity> {
        self.name_to_entity.get(name)
    }

    pub fn entity_from_uid(&self, uid : &Uuid) -> Option<&Entity> {
        self.uid_to_entity.get(uid)
    }

    pub fn uid_from_entity(&self, entity : &Entity) -> Option<&Uuid> {
        self.entity_to_uid.get(entity)
    }
}
