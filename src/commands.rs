use crate::entity_components::Direction;

pub enum MovementCommand {
    Move(Direction),
}

pub enum ActionCommand {
    Shoot,
}
