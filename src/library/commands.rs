use sdl2::mouse::MouseState;

use crate::{entity_components::Direction, Position};

#[derive(Debug)]
pub enum MovementCommand {
    Move(Direction),
}

pub enum ActionCommand {
    Shoot(Direction, Position),
}

pub enum MouseCommand {
    Cmd(MouseState),
}
