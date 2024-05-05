use crate::rect;
use sdl2::rect::Rect;
use specs::prelude::*;

use crate::commands::*;

use crate::entity_components::*;
use crate::entity_flags::*;

pub struct ActionSys;

impl<'a> System<'a> for ActionSys {
    type SystemData = (
        ReadExpect<'a, Option<ActionCommand>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let action = match &*data.0 {
            Some(action) => action,
            None => return,
        };

        let entity = &data.4;
        let updater = &data.5;

        (&data.1, &data.2, &data.3)
            .par_join()
            .for_each(|(_, pos, vel)| match action {
                ActionCommand::Shoot(dir) => match dir {
                    Direction::MoveDelta { x, y } => {
                        let bullet = entity.create();
                        let mut b_vel: Velocity = Velocity {
                            speed: 0,
                            max_speed: 100,
                            acc: 30,
                            last_dir: None,
                        };

                        let mut x_tmp = *x;
                        if *x == 0 && *y == 0 {
                            match vel.last_dir {
                                Some(vel_dir) => match vel_dir {
                                    Direction::MoveDelta { x: vel_x, y: _ } => {
                                        x_tmp = vel_x;
                                    }
                                },
                                None => {
                                    x_tmp = 1;
                                }
                            }
                        }

                        b_vel.encode_speed(x_tmp * (b_vel.acc as i8), y * (b_vel.acc as i8));

                        updater.insert(bullet, b_vel);
                        updater.insert(
                            bullet,
                            SpriteVec {
                                sprite_vec: vec![Sprite {
                                    spritesheet: 1,
                                    region: rect!(0, 0, 5, 5),
                                    mouse_rot_flag: false,
                                    rotation: None,
                                }],
                            },
                        );
                        updater.insert(bullet, DecayLife { life: 10 });
                        updater.insert(bullet, pos.clone());
                        updater.insert(
                            bullet,
                            Damage {
                                dmg: 10,
                                dmg_box: rect!(0, 0, 5, 5),
                            },
                        );
                        updater.insert(bullet, SingleDamage);
                    }
                },
            });
    }
}
