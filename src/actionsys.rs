use specs::prelude::*;

use sdl2::rect::Rect;

use crate::commands::ActionCommand;
use crate::sprite_components::*;

macro_rules! rect( ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

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
            .for_each(|(_, pos, vel)| {
                let mut frames = Vec::<Sprite>::new();
                frames.push(Sprite {
                    spritesheet: 1,
                    region: rect!(0, 0, 5, 5),
                });
                let player_ani = MovementAnimation {
                    current_frame: 0,
                    up_frames: frames.clone(),
                    down_frames: frames.clone(),
                    left_frames: frames.clone(),
                    right_frames: frames.clone(),
                };
                match action {
                    ActionCommand::Shoot => {
                        let bullet = entity.create();
                        updater.insert(
                            bullet,
                            Velocity {
                                speed: 10,
                                direction: vel.direction,
                            },
                        );
                        updater.insert(bullet, Life { life: 30.0 });
                        updater.insert(bullet, player_ani.right_frames[0].clone());
                        updater.insert(bullet, player_ani);
                        updater.insert(bullet, pos.clone());
                    }
                }
            });
        /*
        .with(pos.clone())
        .with(Velocity {
            speed: 10,
            direction: Direction::Right,
        })
        .with(player_ani.right_frames[0].clone())
        .with(player_ani)
        .build();*/
    }
}
