use std::collections::HashSet;

#[derive(Default)]
pub struct GlobalNameRegistry(pub HashSet<String>);
