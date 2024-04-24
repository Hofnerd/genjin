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
        WriteStorage<'a, GravityAfflicted>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (
            &data.0,
            &data.1,
            &mut data.2,
            &data.3,
            (&mut data.4).maybe(),
        )
            .par_join()
            .filter(|(_, _, vel, _, _)| vel.speed != 0)
            .for_each(|(pos, coll, vel, entity, mut grav)| {
                let x_speed: i8 = (vel.speed & 0xff) as i8;
                let y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
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

                    if cur_rect_offset.has_intersection(trect) {
                        vel.speed = 0;

                        match trect.intersect_line(
                            cur_rect_offset.bottom_left(),
                            cur_rect_offset.bottom_right(),
                        ) {
                            Some(_) => match &mut grav {
                                Some(grav) => {
                                    grav.grounded = true;
                                }
                                None => continue,
                            },
                            None => continue,
                        }
                    }
                }
            });
    }
}
