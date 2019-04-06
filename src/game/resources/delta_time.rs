use std::time::Duration;

pub struct DeltaTime(pub Duration);

impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime(Duration::default())
    }
}
