use specs::prelude::*;

use crate::sprite_components::*;

pub struct DecaySys;

/*macro_rules! ( ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x a i32, $y as i32, $w as u32, $h as u32)
    )
);*/

impl<'a> System<'a> for DecaySys {
    type SystemData = (Entities<'a>, WriteStorage<'a, Life>);

    fn run(&mut self, (entities, mut life): Self::SystemData) {
        (&entities, &mut life).par_join().for_each(|(e, life)| {
            if life.life < 0.0 {
                let _ = entities.delete(e);
            } else {
                life.life -= 1.0;
            }
        });
    }
}
