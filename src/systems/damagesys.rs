use sdl2::rect::{Point, Rect};
use specs::prelude::*;

use crate::{entity_components::*, SingleDamage};

pub struct DamageSys;

impl<'a> System<'a> for DamageSys {
    type SystemData = (
        WriteStorage<'a, Health>,
        ReadStorage<'a, Damage>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Position>,
        Entities<'a>,
        ReadStorage<'a, SingleDamage>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let entities = &data.4;
        (&mut data.0, &data.3, &data.4)
            .par_join()
            .for_each(|(health, pos, entity)| {
                let hurt_box =
                    Rect::from_center(pos.point, health.hurt_box.width(), health.hurt_box.height());

                for (dmg, vel, pos, sdflag, tent) in
                    (&data.1, &data.2, &data.3, (&data.5).maybe(), &data.4).join()
                {
                    let dmg_box =
                        Rect::from_center(pos.point, dmg.dmg_box.width(), dmg.dmg_box.height());
                    let (dmg_x_off, dmg_y_off) = vel.unencode_speed();
                    let dmg_box_offset = Rect::from_center(
                        pos.point.offset(dmg_x_off as i32, dmg_y_off as i32),
                        dmg.dmg_box.width(),
                        dmg.dmg_box.height(),
                    );
                    let mut line_vec: Vec<(Point, Point)> = Vec::<(Point, Point)>::new();
                    line_vec.push((dmg_box.bottom_left(), dmg_box_offset.bottom_left()));
                    line_vec.push((dmg_box.bottom_right(), dmg_box_offset.bottom_right()));
                    line_vec.push((dmg_box.top_left(), dmg_box_offset.top_left()));
                    line_vec.push((dmg_box.top_right(), dmg_box_offset.top_right()));

                    let mut hit = false;
                    for (p1, p2) in line_vec {
                        match hurt_box.intersect_line(p1, p2) {
                            Some(_) => {
                                hit = true;
                            }
                            None => {}
                        }
                    }

                    if hit {
                        health.hp -= dmg.dmg;
                        match sdflag {
                            Some(_) => {
                                let _ = entities.delete(tent);
                            }
                            None => {}
                        }
                    }
                }

                if health.hp <= 0 {
                    let _ = entities.delete(entity);
                }
            });
    }
}
