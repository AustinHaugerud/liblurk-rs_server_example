use specs::{Component, VecStorage};
use game::actor::stat::Stat;

pub struct StatShop {
    transfer_magnitude : u16, // How much gold to convert to a stat.
    target : Stat,
}

impl Component for StatShop {
    type Storage = VecStorage<Self>;
}
