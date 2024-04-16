use specs::prelude::*;

use crate::sprite_components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        (&mut data.0, &mut data.1, &data.2)
            .par_join()
            .filter(|(_, _, vel)| vel.speed != 0)
            .for_each(|(anim, sprite, vel)| {
                let frames = match vel.direction {
                    Left => &anim.left_frames,
                    Right => &anim.right_frames,
                    Up => &anim.up_frames,
                    Down => &anim.down_frames,
                };

                anim.current_frame = (anim.current_frame + 1) % frames.len();

                *sprite = frames[anim.current_frame].clone();
            });
    }
}
