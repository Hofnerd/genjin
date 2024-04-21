use specs::prelude::*;

use crate::commands::*;
use crate::entity_components::*;
use crate::entity_flags::*;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        (&data.1, &mut data.2)
            .par_join()
            .for_each(|(_, vel)| match movement_command {
                &MovementCommand::Move(dir) => match dir {
                    Direction::MoveDelta { x_delta, y_delta } => {
                        let (mut x_cur, mut y_cur) = unencode_speed(vel.speed);

                        if x_cur <= 126 || x_cur >= -127 {
                            x_cur = x_cur + x_delta;
                        }
                        if y_cur <= 126 || y_cur >= -127 {
                            y_cur = y_cur + y_delta;
                        }
                        vel.speed = encode_speed(x_cur, y_cur);
                    }
                },
            });
    }
}
