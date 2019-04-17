use game::actor::stat::Stat;
use specs::{Component, VecStorage};

pub struct StatShop {
    pub transfer_magnitude: u16, // How much gold to convert to a stat.
    pub target: Stat,
}

impl Component for StatShop {
    type Storage = VecStorage<Self>;
}
