use specs::prelude::*;

use crate::entity_components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &mut data.1)
            .par_join()
            .for_each(|(pos, vel)| {
                let mut x_speed: i8 = (vel.speed & 0xff) as i8;
                let mut y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
                pos.point = pos.point.offset(x_speed as i32, y_speed as i32);
                if x_speed > 0 {
                    x_speed -= 1;
                } else if x_speed < 0 {
                    x_speed += 1;
                }

                if y_speed > 0 {
                    y_speed -= 1;
                } else if y_speed < 0 {
                    y_speed += 1;
                }
                vel.speed = encode_speed(x_speed, y_speed);
            });
    }
}
