use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use game::resources::events::CharacterEvent;

#[derive(Default)]
pub struct CharacterPrep(pub HashMap<Uuid, CharacterEvent>);
