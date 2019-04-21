use std::collections::HashSet;

#[derive(Default)]
pub struct GlobalNameRegistry {
    names_set: HashSet<String>,
}

impl GlobalNameRegistry {
    pub fn put<T>(&mut self, name: T) -> bool
    where
        T: ToString,
    {
        let name_str = name.to_string();
        if self.names_set.contains(&name_str) {
            false
        } else {
            self.names_set.insert(name_str);
            true
        }
    }
}
