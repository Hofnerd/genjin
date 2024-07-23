mod entities;
mod library;
mod systems;

use collisionsys::CollisionSys;
use entities::*;
use library::globalcomponents::*;
use library::*;
use sdl2::rect::Rect;
use std::time::Duration;
use systems::actionsys::ActionSys;
use systems::damagesys::DamageSys;
use systems::decaysys::DecaySys;
use systems::gravitysys::GravitySys;
use systems::mousesys::MouseSys;
use systems::projectilesys::ProjectileSys;
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
pub const SIM_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 120);
pub const FRAME_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 60);

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
        .with(ProjectileSys, "ProjectileSys", &[])
        .with(GravitySys, "GravitySys", &["Keyboard", "ProjectileSys"])
        .with(
            CollisionSys,
            "CollisionSys",
            &["Keyboard", "ProjectileSys", "GravitySys"],
        )
        .with(
            Physics,
            "Physics",
            &["Keyboard", "ProjectileSys", "GravitySys", "CollisionSys"],
        )
        .with(DecaySys, "DecaySys", &[])
        .with(DamageSys, "DamageSys", &[])
        .with(ActionSys, "ActionSys", &[])
        .with(MouseSys, "MouseSys", &[])
        .build();

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    let action_command: Option<ActionCommand> = None;
    let mouse_command: Option<MouseCommand> = None;

    let screeninfo = Some(ScreenInfo {
        screen_size: ScreenSize::Size {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        },
    });

    world.insert(movement_command);
    world.insert(action_command);
    world.insert(mouse_command);
    world.insert(screeninfo);

    let textures = [
        tc.load_texture("assets/reaper.png")?,
        tc.load_texture("assets/bullet.png")?,
        tc.load_texture("assets/block.png")?,
    ];
    let mut sprite_vec = Vec::<Sprite>::new();
    sprite_vec.push(Sprite {
        spritesheet: 0,
        region: rect!(0, 0, 26, 36),
        mouse_rot_flag: false,
        rotation: None,
    });
    sprite_vec.push(Sprite {
        spritesheet: 2,
        region: rect!(0, 0, 30, 10),
        mouse_rot_flag: true,
        rotation: Some(Rotation {
            rise: 0.0,
            run: 0.0,
            rotation: 0.0,
            rot_point: Some(Point::new(0, 0)),
        }),
    });

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
        .with(SpriteVec { sprite_vec })
        .with(Collideable {
            col_box: rect!(10, 10, 16, 36),
        })
        .with(Player)
        .with(Health {
            hp: 100,
            hurt_box: rect!(10, 10, 16, 36),
        })
        .with(TempTestFlag)
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

        let m_state = event_pump.mouse_state();

        let mut action: Option<ActionCommand> = None;
        if shoot_flag {
            action = Some(ActionCommand::Shoot(
                Direction::MoveDelta {
                    x: x_ctrl,
                    y: y_ctrl,
                },
                Position {
                    point: Point::new(m_state.x(), m_state.y()),
                },
            ));
        }

        let movement_command = Some(MovementCommand::Move(Direction::MoveDelta {
            x: x_ctrl,
            y: y_ctrl,
        }));

        *world.write_resource() = movement_command;
        *world.write_resource() = action;
        *world.write_resource() = Some(MouseCommand::Cmd(m_state));

        dispatcher.dispatch(&mut world);
        world.maintain();

        renderer::render(
            &mut canvas,
            Color::RGB(255, 255, 255),
            &textures,
            world.system_data(),
        )?;

        ::std::thread::sleep(FRAME_RATE);
    }

    return Ok(());
}
