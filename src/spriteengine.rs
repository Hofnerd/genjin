use std::path::PathBuf;

use sdl2::rect::{Point, Rect};

pub const PLAYER_MOVE_SPEED: i32 = 10;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub position: Point,
    pub img: Rect,
    pub path: PathBuf,
    pub speed: i32,
    pub dir: Direction,
}

impl Sprite {
    pub fn new(pos: Point, rec: Rect, path: PathBuf, speed: i32) -> Sprite {
        return Sprite {
            position: pos,
            img: rec,
            path,
            speed,
            dir: Direction::Right,
        };
    }

    pub fn update_sprite(&mut self) {
        match self.dir {
            Direction::Up => {
                self.position = self.position.offset(0, -self.speed);
            }
            Direction::Left => {
                self.position = self.position.offset(-self.speed, 0);
            }
            Direction::Down => {
                self.position = self.position.offset(0, self.speed);
            }
            Direction::Right => {
                self.position = self.position.offset(self.speed, 0);
            }
        }
    }
}
