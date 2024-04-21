use sdl2::rect::Rect;
use specs::prelude::*;

use crate::entity_components::*;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collideable>,
        WriteStorage<'a, Velocity>,
        Entities<'a>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let (tpos, tcoll, tentities) = (&data.0, &data.1, &data.3).clone();
        (&data.0, &data.1, &mut data.2, &data.3)
            .par_join()
            .filter(|(_, _, vel, _)| vel.speed != 0)
            .for_each(|(pos, coll, vel, entity)| {
                let cur_rect =
                    Rect::from_center(pos.point, coll.col_box.width(), coll.col_box.height());
                for (tposi, tcolli, _) in (tpos, tcoll, tentities)
                    .join()
                    .filter(|(_, _, tentitiyi)| entity.id() != tentitiyi.id())
                    .filter(|(tposi, _, _)| tposi.quadrant != pos.quadrant)
                {
                    let mut x_dir = (vel.speed & 0xff) as i8;
                    let mut y_dir = ((vel.speed >> 8) & 0xff) as i8;

                    if x_dir > 0 {
                        if tposi.point.x > pos.point.x {
                            let trect = Rect::from_center(
                                tposi.point,
                                tcolli.col_box.width(),
                                tcolli.col_box.height(),
                            );

                            if cur_rect.has_intersection(trect) {
                                x_dir = 0;
                            }
                        }
                    } else if x_dir < 0 {
                        if tposi.point.x < pos.point.x {
                            let trect = Rect::from_center(
                                tposi.point,
                                tcolli.col_box.width(),
                                tcolli.col_box.height(),
                            );

                            if cur_rect.has_intersection(trect) {
                                x_dir = 0;
                            }
                        }
                    }

                    if y_dir > 0 {
                        if tposi.point.y > pos.point.y {
                            let trect = Rect::from_center(
                                tposi.point,
                                tcolli.col_box.width(),
                                tcolli.col_box.height(),
                            );

                            if cur_rect.has_intersection(trect) {
                                y_dir = 0;
                            }
                        }
                    } else if y_dir < 0 {
                        if tposi.point.y < pos.point.y {
                            let trect = Rect::from_center(
                                tposi.point,
                                tcolli.col_box.width(),
                                tcolli.col_box.height(),
                            );

                            if cur_rect.has_intersection(trect) {
                                y_dir = 0;
                            }
                        }
                    }

                    vel.speed = encode_speed(x_dir, y_dir);
                }
            });
    }
}
