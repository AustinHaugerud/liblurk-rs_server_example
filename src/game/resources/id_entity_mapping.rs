use specs::Entity;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct IdEntityMapping(pub HashMap<Uuid, Entity>);
