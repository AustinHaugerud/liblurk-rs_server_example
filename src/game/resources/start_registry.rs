use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default)]
pub struct StartRegistry(pub HashSet<Uuid>);
