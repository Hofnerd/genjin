use sdl2::rect::Point;
use specs::prelude::*;

use crate::{entity_components::*, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &data.1).par_join().for_each(|(pos, vel)| {
            let x_speed: i8 = (vel.speed & 0xff) as i8;
            let y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
            pos.point = pos.point.offset(x_speed as i32, y_speed as i32);
            pos.point = Point::new(
                pos.point.x % (WINDOW_WIDTH as i32),
                pos.point.y % (WINDOW_HEIGHT as i32),
            );

            if (pos.point.x as u32) < (WINDOW_WIDTH / 2)
                && (pos.point.y as u32) < (WINDOW_HEIGHT / 2)
            {
                pos.quadrant = Quadrant::Q1;
            } else if (pos.point.x as u32) >= (WINDOW_WIDTH / 2)
                && (pos.point.y as u32) < (WINDOW_HEIGHT / 2)
            {
                pos.quadrant = Quadrant::Q2;
            } else if (pos.point.x as u32) < (WINDOW_WIDTH / 2)
                && (pos.point.y as u32) >= (WINDOW_HEIGHT / 2)
            {
                pos.quadrant = Quadrant::Q3;
            } else {
                pos.quadrant = Quadrant::Q4;
            }
        });
    }
}
