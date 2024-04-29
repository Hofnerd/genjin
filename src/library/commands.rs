use sdl2::mouse::MouseState;

use crate::entity_components::Direction;

#[derive(Debug)]
pub enum MovementCommand {
    Move(Direction),
}

pub enum ActionCommand {
    Shoot(Direction),
}

pub enum MouseCommand {
    Cmd(MouseState),
}
