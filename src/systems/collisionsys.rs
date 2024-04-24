use sdl2::rect::Rect;
use specs::prelude::*;

use crate::entity_components::*;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Collideable>,
        WriteStorage<'a, Velocity>,
        Entities<'a>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&data.0, &data.1, &mut data.2, &data.3)
            .par_join()
            .filter(|(_, _, vel, _)| vel.speed != 0)
            .for_each(|(pos, coll, vel, entity)| {
                let x_speed: i8 = (vel.speed & 0xff) as i8;
                let y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
                let cur_rect =
                    Rect::from_center(pos.point, coll.col_box.width(), coll.col_box.height());
                let cur_rect_offset = Rect::from_center(
                    pos.point.offset(x_speed as i32, y_speed as i32),
                    coll.col_box.width(),
                    coll.col_box.height(),
                );

                for (tposi, tcolli, _) in (&data.0, &data.1, &data.3)
                    .join()
                    .filter(|(_, _, tentitiyi)| entity.id() != tentitiyi.id())
                {
                    let trect = Rect::from_center(
                        tposi.point,
                        tcolli.col_box.width(),
                        tcolli.col_box.height(),
                    );

                    match cur_rect_offset.intersection(trect) {
                        Some(rect) => {
                            let (mut x_speed, mut y_speed) = unencode_speed(vel.speed);
                            /* Closer, but not perfect. need to not use speed
                             * and instead use something else to determine
                             * the offset required*/
                            println!("{:?}", rect);

                            if x_speed > 0 {
                                x_speed = (trect.left() - cur_rect.right()) as i8;
                            } else if x_speed < 0 {
                                x_speed = (trect.right() - cur_rect.left()) as i8;
                            }
                            if y_speed > 0 {
                                y_speed = (trect.top() - cur_rect.bottom()) as i8;
                            } else if y_speed < 0 {
                                y_speed = (cur_rect.top() - trect.bottom()) as i8;
                            }

                            vel.speed = encode_speed(x_speed, y_speed);
                            vel.collision = true;
                        }
                        None => continue,
                    }
                }
            });
    }
}
