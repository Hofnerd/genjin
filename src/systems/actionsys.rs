use crate::rect;
use sdl2::rect::Rect;
use specs::prelude::*;

use crate::commands::ActionCommand;

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
            .for_each(|(_, pos, _)| match action {
                ActionCommand::Shoot => {
                    let bullet = entity.create();

                    updater.insert(
                        bullet,
                        Velocity {
                            speed: 30,
                            max_speed: 100,
                        },
                    );
                    updater.insert(
                        bullet,
                        Sprite {
                            spritesheet: 1,
                            region: rect!(0, 0, 5, 5),
                        },
                    );
                    updater.insert(bullet, Life { life: 10 });
                    updater.insert(bullet, pos.clone());
                }
            });
    }
}
