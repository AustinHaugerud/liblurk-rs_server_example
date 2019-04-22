use game::resources::events::CharacterEvent;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Default)]
pub struct CharacterPrep(pub HashMap<Uuid, CharacterEvent>);
