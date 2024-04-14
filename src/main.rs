use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;

use gameoflife::GameOfLife;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::gameoflife::CellState;

mod gameoflife;

pub const SQUARE_SIZE: u32 = 15;
pub const GAME_FIELD_WIDTH: u32 = 50;
pub const GAME_FIELD_HEIGHT: u32 = 50;

#[derive(Debug)]
struct GameStateMsg {
    game: GameOfLife,
    frame: u64,
    thread_closed: bool,
}

#[derive(Debug)]
struct EventListMsg {
    events: Vec<Event>,
}

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

// Thread to perform the game operations
fn game_thread(rx: Receiver<EventListMsg>, tx: SyncSender<GameStateMsg>) -> Result<(), String> {
    let mut game = GameOfLife::new();
    let mut frame: u64 = 0;
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
        let gmsg: GameStateMsg = GameStateMsg {
            game: game.clone(),
            frame,
            thread_closed: false,
        };

        let _ = tx.send(gmsg);

        if let gameoflife::GameState::Playing = game.state {
            frame += 1;
        }
    }
    let gmsg: GameStateMsg = GameStateMsg {
        game: game.clone(),
        frame,
        thread_closed: true,
    };

    let _ = tx.send(gmsg);

    return Ok(());
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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
    let (st1, st2) = dummy_texture(&mut canvas, &tc)?;

    let mut event_pump = sdl_context.event_pump()?;
    let _ = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .name("GameThread".to_string())
        .spawn(move || {
            let _ = game_thread(el_rx, gs_tx);
        })
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

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
        let game = recv.game;
        let frame = recv.frame;

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
    }

    let recv = gs_rx.recv().map_err(|e| e.to_string())?;
    if recv.thread_closed {
        return Ok(());
    } else {
        return Err("Thread Fialed to close".to_string());
    }
}
