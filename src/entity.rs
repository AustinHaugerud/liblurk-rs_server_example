pub struct Entity {
    pub name: String,
    pub attack: u16,
    pub defense: u16,
    pub regen: u16,
    pub health: i16,
    pub gold: u16,
    pub location: u16, // the room number
    pub alive: bool,
    pub monster: bool,
    pub desc: String,
}
