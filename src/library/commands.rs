use crate::entity_components::Direction;

#[derive(Debug)]
pub enum MovementCommand {
    Move(Direction),
}

#[derive(Debug)]
pub enum ActionCommand {
    Shoot(Direction),
}
