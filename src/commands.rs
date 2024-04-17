use crate::sprite_components::Direction;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

pub enum ActionCommand {
    Shoot,
}
