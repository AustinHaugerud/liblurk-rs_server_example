use std::collections::HashMap;
use uuid::Uuid;
use specs::Entity;

#[derive(Default)]
pub struct IdEntityMapping(pub HashMap<Uuid, Entity>);
