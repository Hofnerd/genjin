use specs::prelude::*;

use crate::entity_components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &mut data.1, &data.2)
            .par_join()
            .filter(|(_, _, vel)| vel.speed != 0)
            .for_each(|(_anim, _sprite, _vel)| {});
    }
}
