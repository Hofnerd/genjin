use sdl2::rect::Rect;
use specs::prelude::*;

use crate::entity_components::*;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        WriteStorage<'a, Velocity>,
        Entities<'a>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let (tpos, tsprite, tentities) = (&data.0, &data.1, &data.3).clone();
        (&data.0, &data.1, &mut data.2, &data.3)
            .par_join()
            .filter(|(_, _, vel, _)| vel.speed != 0)
            .for_each(|(pos, sprite, _vel, entity)| {
                let _cur_rect =
                    Rect::from_center(pos.point, sprite.region.width(), sprite.region.height());
                for (_tposi, _tspritei, _) in (tpos, tsprite, tentities)
                    .join()
                    .filter(|(_, _, tentitiyi)| entity.id() != tentitiyi.id())
                {
                    /*       match vel.direction {
                        Direction::Left => {
                            if tposi.point.x < pos.point.x {
                                let trect = Rect::from_center(
                                    tposi.point,
                                    tspritei.region.width(),
                                    tspritei.region.height(),
                                );

                                if cur_rect.has_intersection(trect) {
                                    vel.speed = 0;
                                }
                            } else {
                                continue;
                            }
                        }
                        Direction::Right => {
                            if tposi.point.x > pos.point.x {
                                let trect = Rect::from_center(
                                    tposi.point,
                                    tspritei.region.width(),
                                    tspritei.region.height(),
                                );

                                if cur_rect.has_intersection(trect) {
                                    vel.speed = 0;
                                }
                            } else {
                                continue;
                            }
                        }
                    };*/
                }
            });
    }
}
