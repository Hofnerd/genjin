use specs::prelude::*;

use crate::entity_components::*;
use crate::entity_flags::*;

pub struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        ReadStorage<'a, GravityAfflicted>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        (&data.0, &data.1).par_join().for_each(|(_, _vel)| {});
    }
}
