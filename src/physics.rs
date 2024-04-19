use specs::prelude::*;

use crate::entity_components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &data.1).par_join().for_each(|(pos, vel)| {
            let x_speed: i8 = (vel.speed & 0xff) as i8;
            let y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
            pos.point = pos.point.offset(x_speed as i32, y_speed as i32);

            println!("{:?}", pos.point);
        });
    }
}
