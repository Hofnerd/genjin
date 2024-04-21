use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};
use specs::prelude::*;

use crate::entity_components::*;

pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        let screen_position = pos.point + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    return Ok(());
}
