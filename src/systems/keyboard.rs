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
                    Direction::MoveDelta { x, y } => {
                        let (mut x_cur, mut y_cur) = vel.unencode_speed();

                        x_cur = x_cur + (x * (vel.acc as i8));
                        y_cur = y_cur + (y * (vel.acc as i8));

                        if x_cur >= vel.max_speed {
                            x_cur = vel.max_speed;
                        } else if x_cur <= -vel.max_speed {
                            x_cur = -vel.max_speed;
                        }

                        if y_cur >= vel.max_speed {
                            y_cur = vel.max_speed;
                        } else if y_cur <= -vel.max_speed {
                            y_cur = -vel.max_speed;
                        }

                        vel.encode_speed(x_cur, y_cur);

                        if x != 0 {
                            vel.last_dir = Some(dir);
                        }
                    }
                },
            });
    }
}
