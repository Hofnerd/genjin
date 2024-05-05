use sdl2::rect::{Point, Rect};
use specs::prelude::*;

use crate::commands::*;
use crate::entity_components::*;
use crate::entity_flags::*;
use crate::globalcomponents::ScreenInfo;

pub struct MouseSys;

impl<'a> System<'a> for MouseSys {
    type SystemData = (
        ReadExpect<'a, Option<MouseCommand>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SpriteVec>,
        ReadStorage<'a, TempTestFlag>,
        ReadExpect<'a, Option<ScreenInfo>>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let m_cmd = match &*data.0 {
            Some(m_cmd) => m_cmd,
            None => return,
        };

        let screen_info = match &*data.4 {
            Some(s_info) => s_info,
            None => return,
        };

        (&data.1, &mut data.2, &data.3)
            .par_join()
            .for_each(|(pos, sprites, _)| match m_cmd {
                MouseCommand::Cmd(m_state) => {
                    let mut mpnt = Position {
                        point: Point::new(m_state.x(), m_state.y()),
                    };
                    mpnt.translate_coordinate(screen_info.screen_size);

                    for sprite in sprites
                        .sprite_vec
                        .iter_mut()
                        .filter(|sprite| sprite.mouse_rot_flag == true)
                    {
                        let rect = Rect::from_center(
                            pos.point,
                            sprite.region.width(),
                            sprite.region.height(),
                        );
                        match &mut sprite.rotation {
                            Some(rot) => {
                                let point = match rot.rot_point {
                                    Some(p) => {
                                        Point::new(rect.top_left().x + p.x, rect.top_left().y + p.y)
                                    }
                                    None => rect.center(),
                                };

                                let rise: f64 = (mpnt.point.y() as f64) - (point.y() as f64);
                                let run: f64 = (mpnt.point.x() as f64) - (point.x() as f64);

                                rot.rise = rise;
                                rot.run = run;
                                rot.rotation = rise.atan2(run);
                                rot.rotation = rot.rotation.to_degrees();
                            }

                            None => continue,
                        }
                    }
                }
            });
    }
}
