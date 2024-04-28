mod entities;
mod library;
mod systems;

use collisionsys::CollisionSys;
use entities::*;
use library::*;
use sdl2::rect::Rect;
use std::time::Duration;
use systems::actionsys::ActionSys;
use systems::damagesys::DamageSys;
use systems::decaysys::DecaySys;
use systems::gravitysys::GravitySys;
use systems::*;

use commands::*;
use entity_components::*;
use entity_flags::*;
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
pub const REFRESH_RATE: u32 = 60;

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
        .with(DecaySys, "DecaySys", &[])
        .with(DamageSys, "DamageSys", &[])
        .with(ActionSys, "ActionSys", &[])
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
        /*.with(GravityAfflicted {
            max_vel: 20,
            grounded: false,
            grounded_rect: None,
        })*/
        .with(Velocity {
            speed: 0,
            max_speed: 10,
            acc: 3,
            last_dir: None,
        })
        .with(Position {
            point: Point::new(0, 0),
        })
        .with(Sprite {
            spritesheet: 0,
            region: rect!(0, 0, 26, 36),
        })
        .with(Collideable {
            col_box: rect!(10, 10, 16, 36),
        })
        .with(Player)
        .with(Health {
            hp: 100,
            hurt_box: rect!(10, 10, 16, 36),
        })
        .build();

    world
        .create_entity()
        .with(Position {
            point: Point::new(100, 100),
        })
        .with(Collideable {
            col_box: rect!(0, 0, 100, 20),
        })
        .with(Sprite {
            spritesheet: 2,
            region: rect!(0, 0, 100, 20),
        })
        .build();

    world
        .create_entity()
        .with(Position {
            point: Point::new(100, 0),
        })
        .with(Sprite {
            spritesheet: 2,
            region: rect!(0, 0, 10, 20),
        })
        .with(Health {
            hp: 100,
            hurt_box: rect!(0, 0, 10, 20),
        })
        .build();

    // Bound the world so that entities cant leave the system
    world
        .create_entity()
        .with(Position {
            point: Point::new(0, WINDOW_HEIGHT as i32),
        })
        .with(Collideable {
            col_box: rect!(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        })
        .build();

    world
        .create_entity()
        .with(Position {
            point: Point::new(0, -(WINDOW_HEIGHT as i32)),
        })
        .with(Collideable {
            col_box: rect!(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        })
        .build();

    world
        .create_entity()
        .with(Position {
            point: Point::new(-(WINDOW_WIDTH as i32), 0),
        })
        .with(Collideable {
            col_box: rect!(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        })
        .build();

    world
        .create_entity()
        .with(Position {
            point: Point::new(WINDOW_WIDTH as i32, 0),
        })
        .with(Collideable {
            col_box: rect!(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        })
        .build();

    let mut x_ctrl: i8 = 0;
    let mut y_ctrl: i8 = 0;

    'running: loop {
        let mut shoot_flag: bool = false;
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
                    shoot_flag = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    x_ctrl = 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    x_ctrl = -1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    y_ctrl = -1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => y_ctrl = 1,

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
                    x_ctrl = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    y_ctrl = 0;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    y_ctrl = 0;
                }

                _ => {}
            }
        }

        let mut action: Option<ActionCommand> = None;
        if shoot_flag {
            action = Some(ActionCommand::Shoot(Direction::MoveDelta {
                x: x_ctrl,
                y: y_ctrl,
            }));
        }

        let movement_command = Some(MovementCommand::Move(Direction::MoveDelta {
            x: x_ctrl,
            y: y_ctrl,
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
