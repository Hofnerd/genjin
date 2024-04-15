use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use specs::{Builder, World, WorldExt};
use sprite_components::{Position, Velocity};

use crate::sprite_components::Direction;

mod sprite_components;
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
    thread_closed: bool,
}

#[derive(Debug)]
struct EventListMsg {
    events: Vec<Event>,
}

// Thread to perform the game operations
fn game_thread(rx: Receiver<EventListMsg>, tx: SyncSender<GameStateMsg>) -> Result<(), String> {
    let mut world = World::new();

    world
        .create_entity()
        .with(Position {
            point: Point::new(0, 0),
        })
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .build();

    let mut game = GameStateMsg {
        thread_closed: false,
    };

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
                    repeat: false,
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => {}
                _ => {}
            }
        }
        let _ = tx.send(game.clone());
    }
    let _ = tx.send(game.clone());

    return Ok(());
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture) -> Result<(), String> {
    // Set color
    canvas.set_draw_color(color);
    // Clear the current canvas
    canvas.clear();

    let (width, height) = canvas.output_size()?;

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

        let recv = gs_rx.recv().map_err(|e| e.to_string())?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    let recv = gs_rx.recv().map_err(|e| e.to_string())?;
    if recv.thread_closed {
        return Ok(());
    } else {
        return Err("Thread Fialed to close".to_string());
    }
}
