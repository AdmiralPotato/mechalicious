use std::path::PathBuf;

use ftvf::{Metronome, Mode, Reading, RealtimeNowSource};
use psilo_ecs::ecs_iter;
use vectoracious::Context;

use mechalicious_core::components;
use mechalicious_core::GameWorld;

mod model_registry;
use model_registry::ModelRegistry;

fn render(
    context: &mut Context,
    world: &mut GameWorld, // MAKE THIS NOT MUT NEXT TIME SOLRA WILL FIX IT
    model_registry: &mut ModelRegistry,
    phase: f32,
) {
    let mut render = context.begin_rendering_world().unwrap();
    render.clear(0.2, 0.05, 0.1, 0.0);
    println!("\n\x1B[1mWE ARE RENDERING! phase = {phase}\x1B[0m");
    world.with_ecs_world(|ecs_world| {
        for (entity_id, placement, old_placement, visible) in ecs_iter!(
                ecs_world,
                cur components::Placement,
                prev components::Placement,
                cur components::Visible,
        ) {
            render.model(
                model_registry.get_model(visible.model_path),
                &placement.to_phased_transform(&old_placement, phase),
                &[],
                1.0,
            );
            println!(
                "\tEntity: entity_id={entity_id}, placement={placement:?}, visible={visible:?})"
            );
        }
    });
    let mut render = render.begin_ui();
    render.end_rendering();
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let windowbuilder = || video.window("battle girl^H^H^H^H^H^H^H^H^H^H^Hmechalicious", 640, 480);
    let mut vectoracious = vectoracious::Context::initialize(&video, windowbuilder)
        .expect("Couldn't initialize vectoracious. Bummer!");
    let mut should_quit = false;
    let mut world = GameWorld::new();
    let mut metronome = Metronome::new(
        RealtimeNowSource::new(),
        ftvf::Rate::per_second(5, 1), // want 5 ticks per 1 second
        5,                            // accept being up to 5 ticks behind
    );
    let mut model_registry =
        ModelRegistry::new(PathBuf::from("mechalicious-client/data".to_string()));
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
                        Keycode::F4 if keymod.intersects(Mod::LALTMOD | Mod::RALTMOD) => {
                            should_quit = true;
                            break;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        // call `sample` once per batch. not zero times, not two or more times!
        let refresh_rate =
            unsafe { sdl2::video::Window::from_ref(vectoracious.get_window_context().0) }
                .display_mode()
                .map(|x| x.refresh_rate)
                .unwrap_or(60);
        for reading in metronome.sample(Mode::TargetFramesPerSecond(ftvf::Rate::per_second(
            refresh_rate as u32,
            1,
        ))) {
            match reading {
                Reading::Tick => world.tick(),
                Reading::Frame { phase } => {
                    render(&mut vectoracious, &mut world, &mut model_registry, phase)
                }
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
