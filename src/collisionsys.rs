use crate::rect;
use sdl2::rect::Rect;
use specs::prelude::*;

use crate::sprite_components::*;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        WriteStorage<'a, Velocity>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tpos, tsprite, tentities) = (&data.0, &data.1, &data.3).clone();
        (&data.0, &data.1, &data.2, &data.3)
            .par_join()
            .filter(|(_, _, vel, _)| vel.speed != 0)
            .for_each(|(pos, sprite, vel, entity)| {
                let cur_rect = rect!(
                    pos.point.x,
                    pos.point.y,
                    sprite.region.width(),
                    sprite.region.height()
                );
                (tpos, tsprite, tentities)
                    .par_join()
                    .filter(|(_, _, tentityi)| entity.id() != tentityi.id())
                    .for_each(|(tposi, tspritei, _)| {
                        let trect = rect!(
                            tposi.point.x,
                            tposi.point.y,
                            tspritei.region.width(),
                            tspritei.region.height()
                        );

                        if cur_rect.has_intersection(trect) {
                            println!("{:?}{:?}", cur_rect, trect);
                            println!("Intersection");
                        }
                    });
            });
    }
}
