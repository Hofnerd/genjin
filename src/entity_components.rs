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

pub fn generate_speed(x_speed: i8, y_speed: i8) -> u16 {
    let tmp = (((y_speed as u16) << 8) & 0xff00) | ((x_speed as u16) & 0xff);
    return tmp;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    MoveDelta { x_delta: i8, y_delta: i8 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
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
