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
pub struct Collideable {
    pub col_box: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct GravityAfflicted {
    pub max_vel: i32,
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

pub fn encode_speed(x_speed: i8, y_speed: i8) -> u16 {
    let mut x_speed = x_speed;
    if x_speed == 127 {
        x_speed = 127
    } else if x_speed == -128 {
        x_speed = -128;
    }

    let mut y_speed = y_speed;
    if y_speed == 127 {
        y_speed = 127
    } else if y_speed == -128 {
        y_speed = -128;
    }

    let tmp = (((y_speed as u16) << 8) & 0xff00) | ((x_speed as u16) & 0xff);
    return tmp;
}

pub fn unencode_speed(vel: u16) -> (i8, i8) {
    let x_speed: i8 = (vel & 0xff) as i8;
    let y_speed: i8 = ((vel >> 8) & 0xff) as i8;
    return (x_speed, y_speed);
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
