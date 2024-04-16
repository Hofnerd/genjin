use specs::prelude::*;

use crate::sprite_components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        (&mut data.0, &data.1)
            .par_join()
            .for_each(|(pos, vel)| match vel.direction {
                Left => {
                    pos.point = pos.point.offset(-vel.speed, 0);
                }
                Right => {
                    pos.point = pos.point.offset(vel.speed, 0);
                }
                Up => {
                    pos.point = pos.point.offset(0, -vel.speed);
                }
                Down => {
                    pos.point = pos.point.offset(0, vel.speed);
                }
            });
    }
}
