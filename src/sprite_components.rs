use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Life {
    pub life: f32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Health {
    pub hp: u32,
}

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Bullet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
