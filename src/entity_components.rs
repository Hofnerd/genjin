use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point,
    pub quadrant: Quadrant,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Life {
    pub life: f32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    MoveDelta { x_delta: i8, y_delta: i8 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Quadrant {
    Q1,
    _Q2,
    _Q3,
    _Q4,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
