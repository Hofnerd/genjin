mod animator;
mod keyboard;
mod physics;
mod renderer;
mod sprite_components;

use std::time::Duration;

use animator::Animator;
use keyboard::Keyboard;
use physics::Physics;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use specs::prelude::*;
use sprite_components::*;

pub const SQUARE_SIZE: u32 = 15;
pub const GAME_FIELD_WIDTH: u32 = 50;
pub const PLAYER_MOVE_SPEED: i32 = 10;
pub const GAME_FIELD_HEIGHT: u32 = 50;

macro_rules! rect( ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    return match direction {
        Direction::Up => 3,
        Direction::Down => 0,
        Direction::Left => 1,
        Direction::Right => 2,
    };
}

fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();

    let y_off = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);
    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_off,
                frame_width,
                frame_height,
            ),
        });
    }

    return frames;
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

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

    let mut dispatcher = DispatcherBuilder::new()
        .with(Keyboard, "Keyboard", &[])
        .with(Physics, "Physics", &["Keyboard"])
        .with(Animator, "Animator", &[])
        .build();

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    let textures = [tc.load_texture("assets/reaper.png")?];

    let player_spritesheet = 0;
    let player_top_left_frame = rect!(0, 0, 26, 36);
    let player_ani = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position {
            point: Point::new(0, 0),
        })
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(player_ani.right_frames[0].clone())
        .with(player_ani)
        .build();

    let mut i = 0;
    'running: loop {
        let mut movement_command: Option<MovementCommand> = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        renderer::render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            world.system_data(),
        )?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
