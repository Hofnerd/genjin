use sdl2::rect::{Point, Rect};
use std::path::PathBuf;
pub const PLAYER_MOVE_SPEED: i32 = 10;

use crate::sprite_components::*;

pub fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();

    let y_off = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);
    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_off,
                frame_width,
                frame_height,
            ),
        });
    }

    return frames;
}

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub cur_frame: i32,
    pub path: PathBuf,
}

pub fn direction_spritesheet_row(direction: Direction) -> i32 {
    match direction {
        Direction::Up => 3,
        Direction::Down => 0,
        Direction::Left => 1,
        Direction::Right => 2,
    }
}

impl Player {
    pub fn new(pos: Point, rec: Rect, speed: i32, path: PathBuf) -> Player {
        return Player {
            position: pos,
            sprite: rec,
            speed,
            direction: Direction::Right,
            cur_frame: 0,
            path,
        };
    }

    pub fn update_sprite(&mut self) {
        match self.direction {
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

        if self.speed != 0 {
            self.cur_frame = (self.cur_frame + 1) % 3;
        }
    }
}
