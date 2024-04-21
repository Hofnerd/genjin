use specs::prelude::*;

use crate::entity_components::*;
use crate::entity_flags::*;

pub struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        ReadStorage<'a, GravityAfflicted>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&data.0, &mut data.1).par_join().for_each(|(_, vel)| {
            let mut y_cur = ((vel.speed >> 8) & 0xff) as i8;
            y_cur += 1;

            vel.speed = vel.speed & 0x00ff;
            vel.speed = (((y_cur as u16) << 8) & 0xff00) | vel.speed;
        });
    }
}
