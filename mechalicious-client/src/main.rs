use std::path::PathBuf;

use ftvf::{Metronome, Mode, Reading, RealtimeNowSource};
use psilo_ecs::{ecs_get, ecs_iter, EntityId};
use vectoracious::Context;

use mechalicious_core::{components, GameWorld, Transform};

mod model_registry;
use model_registry::ModelRegistry;
use vectoracious::Scale;

struct ClientState {
    camera_state: components::Placement,
    camera_target: components::Placement,
    camera_tracked_entity_id: EntityId,
}

impl ClientState {
    fn tick(&mut self, world: &mut GameWorld) {
        // Update the camera target based on the tracked entity
        world.with_ecs_world(|world| {
            match ecs_get!(world, self.camera_tracked_entity_id, cur components::Placement) {
                Some(x) => self.camera_target.position = x.position,
                _ => {}
            }
        });
        // Update the camera state based on the camera target
        self.camera_state.lerp_toward(&self.camera_target, 0.5);
    }
}

fn render(
    context: &mut Context,
    world: &mut GameWorld, // MAKE THIS NOT MUT NEXT TIME SOLRA WILL FIX IT
    model_registry: &mut ModelRegistry,
    phase: f32,
    client_state: &ClientState,
) {
    let (width, height) = context.get_window().drawable_size();
    let aspect_correction = if width > height {
        // do a thing
        Scale::new(height as f32 / width as f32, 1.0)
    } else {
        // do the sideways of that thing
        Scale::new(1.0, width as f32 / height as f32)
    };
    let aspect_correction = Transform::from_matrix_unchecked(aspect_correction.to_homogeneous());
    let camera_transform = aspect_correction * client_state.camera_state.get_inverse_transform();
    let mut render = context.begin_rendering_world().unwrap();
    render.clear(0.2, 0.05, 0.1, 0.0);
    // println!("\n\x1B[1mWE ARE RENDERING! phase = {phase}\x1B[0m");
    world.with_ecs_world(|ecs_world| {
        for (entity_id, placement, old_placement, visible) in ecs_iter!(
                ecs_world,
                cur components::Placement,
                prev components::Placement,
                cur components::Visible,
        ) {
            render.model(
                model_registry.get_model(visible.model_path),
                &(camera_transform * placement.get_phased_transform(&old_placement, phase)),
                &[],
                1.0,
            );
            // println!(
            //     "\tEntity: entity_id={entity_id}, placement={placement:?}, visible={visible:?})"
            // );
        }
    });
    let mut render = render.begin_ui();
    render.end_rendering();
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let windowbuilder = || video.window("battle girl^H^H^H^H^H^H^H^H^H^H^Hmechalicious", 640, 960);
    let mut vectoracious = vectoracious::Context::initialize(&video, windowbuilder)
        .expect("Couldn't initialize vectoracious. Bummer!");
    let mut should_quit = false;
    let mut world = GameWorld::new();
    let mut metronome = Metronome::new(
        RealtimeNowSource::new(),
        ftvf::Rate::per_second(60, 1), // want 60 ticks per 1 second
        5,                             // accept being up to 5 ticks behind
    );
    let mut model_registry =
        ModelRegistry::new(PathBuf::from("mechalicious-client/data".to_string()));
    let player_id = 3;
    let mut client_state = ClientState {
        camera_state: components::Placement::default(),
        camera_target: components::Placement::default(),
        camera_tracked_entity_id: player_id,
    };
    client_state.camera_state.scale = 16.0;
    client_state.camera_target.scale = 1.0;
    let mut going_left = false;
    let mut going_right = false;
    let mut going_up = false;
    let mut going_down = false;
    let mut controls = components::ShipControls::default();
    while !should_quit {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => {
                    should_quit = true;
                    break;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => {
                    use sdl2::keyboard::{Keycode, Mod};
                    match keycode {
                        Keycode::Escape => {
                            should_quit = true;
                            break;
                        }
                        Keycode::W => going_up = true,
                        Keycode::S => going_down = true,
                        Keycode::A => going_left = true,
                        Keycode::D => going_right = true,
                        Keycode::F4 if keymod.intersects(Mod::LALTMOD | Mod::RALTMOD) => {
                            should_quit = true;
                            break;
                        }
                        _ => (),
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    use sdl2::keyboard::{Keycode, Mod};
                    match keycode {
                        Keycode::W => going_up = false,
                        Keycode::S => going_down = false,
                        Keycode::A => going_left = false,
                        Keycode::D => going_right = false,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        controls.movement.x = if going_left {
            -1.0
        } else if going_right {
            1.0
        } else {
            0.0
        };
        controls.movement.y = if going_down {
            -1.0
        } else if going_up {
            1.0
        } else {
            0.0
        };
        // println!("\n\x1B[1mcontrols = {controls:?}\x1B[0m");
        // call `sample` once per batch. not zero times, not two or more times!
        let refresh_rate = vectoracious
            .get_window()
            .display_mode()
            .map(|x| x.refresh_rate)
            .unwrap_or(60);
        for reading in metronome.sample(Mode::TargetFramesPerSecond(ftvf::Rate::per_second(
            refresh_rate as u32,
            1,
        ))) {
            match reading {
                Reading::Tick => {
                    world.tick(&[(player_id, &controls)]);
                    // do camera???
                    client_state.tick(&mut world);
                }
                Reading::Frame { phase } => render(
                    &mut vectoracious,
                    &mut world,
                    &mut model_registry,
                    phase,
                    &client_state,
                ),
                Reading::TimeWentBackwards => eprintln!("Warning: time flowed backwards!"),
                Reading::TicksLost => eprintln!("Warning: we're too slow, lost some ticks!"),
                // Mode::UnlimitedFrames never returns Idle, but other modes can, and
                // this is the way it should be handled.
                Reading::Idle { duration } => std::thread::sleep(duration),
            }
        }
    }
    println!("Hello, world!");
}
