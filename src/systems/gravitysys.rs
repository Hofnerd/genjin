use specs::prelude::*;

use crate::entity_components::*;

pub struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        ReadStorage<'a, GravityAfflicted>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&data.0, &mut data.1)
            .par_join()
            .filter(|(grav, _)| grav.grounded != true)
            .for_each(|(grav, vel)| {
                let (x_cur, mut y_cur) = vel.unencode_speed();
                y_cur += 1;

                if y_cur as i32 > grav.max_vel {
                    y_cur = grav.max_vel as i8;
                }
                vel.encode_speed(x_cur, y_cur);
            });
    }
}
