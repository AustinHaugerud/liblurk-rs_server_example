use specs::{Component, VecStorage};
use std::time::Duration;

const POINTS_PER_HP : f32 = 100f32;

pub struct Regeneration {
    pub regeneration : u16,
}

impl Regeneration {
    pub fn get_regen_amount(&self, duration : &Duration) -> u16 {
        let time_scalar = duration.as_millis() as f32 * 0.01f32;
        let regen_amount = self.regeneration as f32 * time_scalar * (1f32 / POINTS_PER_HP);
        debug_assert!(regen_amount >= 0f32);
        regen_amount as u16
    }
}

impl Component for Regeneration {
    type Storage = VecStorage<Self>;
}
