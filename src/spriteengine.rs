use std::path::PathBuf;

use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone)]
pub struct Sprite {
    pub position: Point,
    pub img: Rect,
    pub path: PathBuf,
    pub speed: i32,
}

impl Sprite {
    pub fn new(pos: Point, rec: Rect, path: PathBuf, speed: i32) -> Sprite {
        return Sprite {
            position: pos,
            img: rec,
            path,
            speed,
        };
    }
}
