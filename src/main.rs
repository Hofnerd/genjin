mod collisionsys;
mod commands;
mod entity_components;
mod entity_flags;
mod gravitysys;
mod keyboard;
mod macros;
mod physics;
mod renderer;

use collisionsys::CollisionSys;
use gravitysys::GravitySys;
use sdl2::rect::Rect;
use std::time::Duration;

use commands::*;
use entity_components::*;
use entity_flags::{GravityAfflicted, KeyboardControlled};
use keyboard::Keyboard;
use physics::Physics;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::TextureCreator;
use specs::prelude::*;

pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_WIDTH: u32 = 800;
pub const REFRESH_RATE: u32 = 120;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Rust SDL2 Demo: Game of life", WINDOW_WIDTH, WINDOW_HEIGHT)
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
        .with(GravitySys, "GravitySys", &["Keyboard"])
        .with(CollisionSys, "CollisionSys", &["Keyboard", "GravitySys"])
        .with(
            Physics,
            "Physics",
            &["Keyboard", "GravitySys", "CollisionSys"],
        )
        //.with(ActionSys, "ActionSys", &["Keyboard"])
        //.with(Animator, "Animator", &[])
        //.with(DecaySys, "DecaySys", &[])
        .build();

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    let action_command: Option<ActionCommand> = None;
    world.insert(movement_command);
    world.insert(action_command);

    let textures = [
        tc.load_texture("assets/reaper.png")?,
        tc.load_texture("assets/bullet.png")?,
        tc.load_texture("assets/block.png")?,
    ];

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(GravityAfflicted)
        .with(Velocity { speed: 0 })
        .with(Position {
            point: Point::new(0, 0),
            quadrant: Quadrant::Q1,
        })
        .with(Sprite {
            spritesheet: 0,
            region: rect!(0, 0, 26, 36),
        })
        .build();

    let mut x_ctrl: i8 = 0;
    let mut y_ctrl: i8 = 0;

    'running: loop {
        let mut action: Option<ActionCommand> = None;

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
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    action = Some(ActionCommand::Shoot);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    x_ctrl = 3;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    x_ctrl = -3;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    y_ctrl = -3;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    x_ctrl = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    x_ctrl -= 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    y_ctrl = 0;
                }
                _ => {}
            }
        }

        let movement_command = Some(MovementCommand::Move(Direction::MoveDelta {
            x_delta: x_ctrl,
            y_delta: y_ctrl,
        }));

        *world.write_resource() = movement_command;
        *world.write_resource() = action;

        dispatcher.dispatch(&mut world);
        world.maintain();

        renderer::render(
            &mut canvas,
            Color::RGB(255, 255, 255),
            &textures,
            world.system_data(),
        )?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / REFRESH_RATE));
    }

    return Ok(());
}
