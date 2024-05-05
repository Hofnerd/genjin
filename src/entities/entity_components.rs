use crate::ScreenSize;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point,
}

impl Position {
    pub fn translate_coordinate(&mut self, screen: ScreenSize) {
        match screen {
            ScreenSize::Size {
                width: w,
                height: h,
            } => {
                self.point = self.point - Point::new(w as i32 / 2, h as i32 / 2);
            }
        };
    }
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
    pub grounded: bool,
    pub grounded_rect: Option<Rect>,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct DecayLife {
    pub life: u32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Health {
    pub hp: u32,
    pub hurt_box: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Damage {
    pub dmg: u32,
    pub dmg_box: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: u16,
    pub max_speed: i8,
    pub acc: u32,
    pub last_dir: Option<Direction>,
}

impl Velocity {
    pub fn encode_speed(&mut self, x_speed: i8, y_speed: i8) {
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
        self.speed = tmp;
    }

    pub fn unencode_speed(&self) -> (i8, i8) {
        let vel = self.speed;
        let x_speed: i8 = (vel & 0xff) as i8;
        let y_speed: i8 = ((vel >> 8) & 0xff) as i8;
        return (x_speed, y_speed);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    MoveDelta { x: i8, y: i8 },
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
    pub mouse_rot_flag: bool,
    pub rotation: Option<Rotation>,
}

#[derive(Debug, Clone)]
pub struct Rotation {
    pub rise: f64,
    pub run: f64,
    pub rotation: f64,
    pub rot_point: Option<Point>,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct SpriteVec {
    pub sprite_vec: Vec<Sprite>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
