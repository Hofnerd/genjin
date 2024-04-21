use specs::prelude::*;

use crate::entity_components::*;
use crate::entity_flags::*;

pub struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        ReadStorage<'a, GravityAfflicted>,
        ReadStorage<'a, Grounded>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&data.0, !&data.1, &mut data.2)
            .par_join()
            .for_each(|(grav, _, vel)| {
                let (x_cur, mut y_cur) = unencode_speed(vel.speed);
                y_cur += 1;

                if y_cur as i32 > grav.max_vel {
                    y_cur = grav.max_vel as i8;
                }
                vel.speed = encode_speed(x_cur, y_cur);
            });
    }
}
