use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use spriteengine::Sprite;

mod gameoflife;
mod spriteengine;

pub const SQUARE_SIZE: u32 = 15;
pub const GAME_FIELD_WIDTH: u32 = 50;
pub const GAME_FIELD_HEIGHT: u32 = 50;

macro_rules! rect( ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Debug, Clone)]
struct GameStateMsg {
    sprites: Vec<Sprite>,
    thread_closed: bool,
}

#[derive(Debug)]
struct EventListMsg {
    events: Vec<Event>,
}

/*fn dummy_texture<'a>(
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
}*/

// Thread to perform the game operations
fn game_thread(rx: Receiver<EventListMsg>, tx: SyncSender<GameStateMsg>) -> Result<(), String> {
    //  let mut game = GameOfLife::new();
    let mut game = GameStateMsg {
        sprites: Vec::<Sprite>::new(),
        thread_closed: false,
    };

    game.sprites.push(Sprite::new(
        Point::new(0, 0),
        rect!(48, 61, 22, 67),
        "assets/fire_wizard/Walk.png".into(),
        10,
    ));

    'running: loop {
        let recv = rx.recv().map_err(|e| e.to_string())?;
        for event in recv.events.into_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    let mut sp = game.sprites.pop().unwrap();
                    sp.position = sp.position.offset(0, -sp.speed);
                    game.sprites.push(sp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let mut sp = game.sprites.pop().unwrap();
                    sp.position = sp.position.offset(-sp.speed, 0);
                    game.sprites.push(sp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let mut sp = game.sprites.pop().unwrap();
                    sp.position = sp.position.offset(0, sp.speed);
                    game.sprites.push(sp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let mut sp = game.sprites.pop().unwrap();
                    sp.position = sp.position.offset(sp.speed, 0);
                    game.sprites.push(sp);
                }
                _ => {}
            }
        }

        let _ = tx.send(game.clone());
    }
    game.thread_closed = true;
    let _ = tx.send(game.clone());

    return Ok(());
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    sprite: &Sprite,
) -> Result<(), String> {
    // Set color
    canvas.set_draw_color(color);
    // Clear the current canvas
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // PERFORM RENDERING
    let screen_pos = sprite.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_pos, sprite.img.width(), sprite.img.height());
    canvas.copy(texture, sprite.img, screen_rect)?;

    // Render
    canvas.present();
    return Ok(());
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let (gs_tx, gs_rx) = mpsc::sync_channel::<GameStateMsg>(10);
    let (el_tx, el_rx) = mpsc::sync_channel::<EventListMsg>(10);

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

    let tc: TextureCreator<_> = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    let _ = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .name("GameThread".to_string())
        .spawn(move || {
            let _ = game_thread(el_rx, gs_tx);
        })
        .map_err(|e| e.to_string())?;

    'running: loop {
        let mut events: Vec<Event> = Vec::<Event>::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    events = vec![event.clone()];
                    let elmsg = EventListMsg { events };
                    let _ = el_tx.send(elmsg);
                    break 'running;
                }

                _ => events.push(event),
            }
        }

        let elmsg = EventListMsg { events };
        let _ = el_tx.send(elmsg);

        let mut recv = gs_rx.recv().map_err(|e| e.to_string())?;

        let sp = recv.sprites.pop().unwrap();
        let img_texture = tc.load_texture(&sp.path)?;

        render(&mut canvas, Color::RGB(0, 0, 0), &img_texture, &sp)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    let recv = gs_rx.recv().map_err(|e| e.to_string())?;
    if recv.thread_closed {
        return Ok(());
    } else {
        return Err("Thread Fialed to close".to_string());
    }
}
