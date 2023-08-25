use ftvf::{Metronome, Mode, RealtimeNowSource, Status};
use psilo_ecs::ecs_iter;

use mechalicious_core::components;
use mechalicious_core::GameWorld;

fn render(world: &GameWorld, phase: f32) {
    std::thread::sleep(std::time::Duration::from_millis(200));
    // ??? phase
    // world.???
    println!("\n\x1B[1mWE ARE RENDERING! phase = {phase}\x1B[0m");
    let ecs_world = world.get_ecs_world();
    for (entity_id, position, visible) in
        ecs_iter!(ecs_world, cur components::Position, cur components::Visible)
    {
        println!("\tEntity: entity_id={entity_id}, position={position:?}, visible={visible:?})");
    }
}

fn main() {
    let mut should_quit = false;
    let mut world = GameWorld::new();
    let mut metronome = Metronome::new(
        RealtimeNowSource::new(),
        (5, 1), // want 30 ticks per 1 second
        5,
    ); // accept being up to 5 ticks behind
    while !should_quit {
        //TODO:world.handle_input();
        // call `sample` once per batch. not zero times, not two or more times!
        metronome.sample();
        while let Some(status) = metronome.status(Mode::MaxOneFramePerTick) {
            match status {
                Status::Tick => world.tick(),
                Status::Frame { phase } => render(&world, phase),
                Status::TimeWentBackwards => eprintln!("Warning: time flowed backwards!"),
                Status::TicksLost(n) => eprintln!("Warning: we're too slow, lost {} ticks!", n),
                // No special handling or warning message is needed for Rollover. In
                // practice, it will never be seen.
                Status::Rollover => (),
                // Mode::UnlimitedFrames never returns Idle, but other modes can, and
                // this is the way it should be handled.
                Status::Idle => metronome.sleep_until_next_tick(),
            }
        }
    }
    println!("Hello, world!");
}
