use specs::Entity;

pub struct MoveTask {
    pub mover: Entity,
    pub target: Entity,
}

#[derive(Default)]
pub struct MoveTasks(pub Vec<MoveTask>);
