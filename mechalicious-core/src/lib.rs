use psilo_ecs::*;

use nalgebra::{point, vector};
pub type Point = nalgebra::Point2<f32>;
pub type Vector = nalgebra::Vector2<f32>;
/*

Note for future Admiral: If you find yourself needing to turn a Point into a
Vector, (for instance, when averaging points) use my_point.coords instead of
my_point.into(). my_point.coords will just give you the coordinates, as a vector. .into() will convert it into a *homogenous* vector, `x, y, 1`.

*/

// if we do: pub mod components
// then our dependencies can use: mechalicious_core::components::Position
// but if we do: pub use components::*
// then our dependencies can use: mechalicious_core::Position

pub mod components;
use components::*;

pub struct GameWorld {
    ecs_world: EcsWorld,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        let mut ecs_world = EcsWorld::with_blank_schema();
        ecs_spawn!(
            ecs_world,
            Position {
                position: point![69.0, 420.0],
                angle: 7.0,
            },
            Physics {
                mass: 1.0,
                moment: 1.0,
                force: vector![0.0, 0.0],
                torque: 0.0,
                velocity: vector![1.0, 0.5],
                angular_velocity: 0.0,
            },
            ShipControls {
                movement: vector![0.0, 0.0],
                aim: vector![0.0, 0.0],
                fire: false
            },
            Visible {
                model_path: "mechalicious.v2d",
            },
        );
        GameWorld { ecs_world }
    }
    pub fn tick(&mut self) {
        self.ecs_world.unbuffered_tick(|world| {
            // this is where our Systems go
            for (_, position, physics) in ecs_iter!(world, mut Position, mut Physics) {
                let linear_acceleration = physics.force / physics.mass;
                let angular_acceleration = physics.torque / physics.moment;
                position.position += physics.velocity + linear_acceleration * 0.5;
                position.angle += physics.angular_velocity + angular_acceleration * 0.5;
                physics.velocity += linear_acceleration;
                physics.angular_velocity += angular_acceleration;
                // zero out the forces for next tick
                physics.force = vector![0.0, 0.0];
                physics.torque = 0.0;
            }
        });
    }
    pub fn get_ecs_world(&self) -> &EcsWorld {
        &self.ecs_world
    }
}
