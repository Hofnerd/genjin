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
        ReadStorage<'a, ProjectileProperties>,
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
            .for_each(|(_, pos, _)| match action {
                ActionCommand::Shoot(dir) => match dir {
                    dir => {
                        let bullet = entity.create();
                        let b_vel: Velocity = Velocity {
                            speed: 0,
                            max_speed: 100,
                            acc: 30,
                            last_dir: None,
                        };

                        updater.insert(bullet, b_vel);
                        updater.insert(
                            bullet,
                            SpriteVec {
                                sprite_vec: vec![Sprite {
                                    spritesheet: 1,
                                    region: rect!(0, 0, 5, 5),
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
                        updater.insert(
                            bullet,
                            ProjectileProperties {
                                direction: dir.clone(),
                                owner: -1,
                            },
                        );
                        updater.insert(bullet, SingleDamage);
                    }
                },
            });
    }
}
