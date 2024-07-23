use specs::prelude::*;

pub struct ProjectileSys;

impl<'a> System<'a> for ProjectileSys {
    type SystemData = (Entities<'a>,);

    fn run(&mut self, _data: Self::SystemData) {}
}
