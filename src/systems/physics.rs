use specs::prelude::*;

use crate::Rect;

use crate::entity_components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, GravityAfflicted>,
        ReadStorage<'a, Collideable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (
            &mut data.0,
            &mut data.1,
            (&mut data.2).maybe(),
            (&mut data.3).maybe(),
        )
            .par_join()
            .for_each(|(pos, vel, grav, coll)| {
                let mut x_speed: i8 = (vel.speed & 0xff) as i8;
                let mut y_speed: i8 = ((vel.speed >> 8) & 0xff) as i8;
                pos.point = pos.point.offset(x_speed as i32, y_speed as i32);
                if x_speed > 0 {
                    x_speed -= 1;
                } else if x_speed < 0 {
                    x_speed += 1;
                }

                if y_speed < 0 {
                    y_speed += 1;
                }

                match grav {
                    Some(grav) => match coll {
                        Some(coll) => {
                            let cur_rect = Rect::from_center(
                                pos.point,
                                coll.col_box.width(),
                                coll.col_box.height(),
                            );

                            match grav.grounded_rect {
                                Some(grect) => {
                                    match grect.intersect_line(
                                        cur_rect.bottom_left(),
                                        cur_rect.bottom_right(),
                                    ) {
                                        Some(_) => {}
                                        None => {
                                            grav.grounded = false;
                                            grav.grounded_rect = None;
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
                    },
                    None => {
                        if y_speed > 0 {
                            y_speed -= 1;
                        }
                    }
                }

                vel.encode_speed(x_speed, y_speed);
            });
    }
}
