use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::gameoflife::CellState;

mod gameoflife;

pub const SQUARE_SIZE: u32 = 40;
pub const GAME_FIELD_WIDTH: u32 = 40;
pub const GAME_FIELD_HEIGHT: u32 = 40;

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>), String> {
    enum TextureColor {
        Yellow,
        White,
    }

    let mut st1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;

    let mut st2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    {
        let textures = vec![
            (&mut st1, TextureColor::Yellow),
            (&mut st2, TextureColor::White),
        ];

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Yellow => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                if (i + j) % 4 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                                if (i + j) % 9 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(200, 200, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                            }
                        }
                    }
                    TextureColor::White => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                if (i + j) % 7 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                                if (i + j) % 5 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                            }
                        }
                    }
                };
                for i in 0..SQUARE_SIZE {
                    for j in 0..SQUARE_SIZE {
                        if (i + j) % 7 == 0 {
                            texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                        if (i + j) % 5 == 0 {
                            texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }

    return Ok((st1, st2));
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Rust SDL2 Demo: Game of life",
            SQUARE_SIZE * GAME_FIELD_WIDTH,
            SQUARE_SIZE * GAME_FIELD_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Render \"{}\"", canvas.info().name);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let tc: TextureCreator<_> = canvas.texture_creator();

    let (st1, st2) = dummy_texture(&mut canvas, &tc)?;

    let mut game = gameoflife::GameOfLife::new();

    let mut event_pump = sdl_context.event_pump()?;

    let mut frame: u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    game.toggle_state();
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let x = (x as u32) / SQUARE_SIZE;
                    let y = (y as u32) / SQUARE_SIZE;
                    match game.get_mut(x as i32, y as i32) {
                        Some(cell) => {
                            cell.toggle_state();
                        }
                        None => unreachable!(),
                    }
                }
                _ => {}
            }
        }

        if frame >= 30 {
            game.update();
            frame = 0;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (i, cell) in (&game).into_iter().enumerate() {
            let i = i as u32;
            let st = if frame >= 15 { &st1 } else { &st2 };
            if cell.state == CellState::Alive {
                canvas.copy(
                    st,
                    None,
                    Rect::new(
                        ((i % GAME_FIELD_WIDTH) * SQUARE_SIZE) as i32,
                        ((i / GAME_FIELD_WIDTH) * SQUARE_SIZE) as i32,
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ),
                )?;
            }
        }
        canvas.present();
        if let gameoflife::GameState::Playing = game.state {
            frame += 1;
        }
    }

    return Ok(());
}
