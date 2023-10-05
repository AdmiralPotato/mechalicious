use std::f32::consts::{PI, TAU};

use psilo_ecs::*;

use arcow::Arcow;
use nalgebra::{point, vector};
pub type Point = nalgebra::Point2<f32>;
pub type Vector = nalgebra::Vector2<f32>;
pub type Transform = nalgebra::Transform2<f32>;
pub type Similarity = nalgebra::Similarity2<f32>;
/*

Note for future Admiral: If you find yourself needing to turn a Point into a
Vector, (for instance, when averaging points) use my_point.coords instead of
my_point.into(). my_point.coords will just give you the coordinates, as a vector. my_point.into() will convert it into a *homogenous* vector, `x, y, 1`.

*/

// if we do: pub mod components
// then our dependencies can use: mechalicious_core::components::Position
// but if we do: pub use components::*
// then our dependencies can also use: mechalicious_core::Position

pub mod components;
use components::*;

pub fn angle_lerp(a: f32, b: f32, theta: f32) -> f32 {
    let delta = b - a;
    let delta = if delta.abs() >= PI {
        if delta > 0.0 {
            delta - TAU
        } else {
            delta + TAU
        }
    } else {
        delta
    };
    a + (delta * theta)
}

pub struct GameWorld {
    prev_ecs_world: Arcow<EcsWorld>,
    ecs_world: Arcow<EcsWorld>,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        let mut ecs_world = EcsWorld::with_blank_schema();
        ecs_spawn!(
            ecs_world,
            Placement {
                position: point![-1.0, -1.0],
                angle: 7.0,
                scale: 0.3,
            },
            Physics {
                mass: 1.0,
                moment: 1.0,
                force: vector![0.0, 0.0],
                torque: 0.0,
                velocity: vector![0.01, 0.01],
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
        ecs_spawn!(
            ecs_world,
            Placement {
                position: point![0.0, 0.0],
                angle: 7.0,
                scale: 0.3,
            },
            Physics {
                mass: 1.0,
                moment: 1.0,
                force: vector![0.0, 0.0],
                torque: 0.0,
                velocity: vector![0.0, 0.0],
                angular_velocity: 0.01,
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
        let ecs_world = Arcow::new(ecs_world);
        GameWorld {
            prev_ecs_world: ecs_world.clone(),
            ecs_world,
        }
    }
    pub fn tick(&mut self, inputs: &[(EntityId, &ShipControls)]) {
        self.prev_ecs_world = self.ecs_world.clone();
        self.ecs_world = self.ecs_world.buffered_tick(|world| {
            // this is where our Systems go
            for (entity_id, player_controls) in inputs {
                if let Some(mut controls) = ecs_get!(world, *entity_id, mut ShipControls) {
                    (*controls).clone_from(player_controls);
                } else {
                    eprintln!("WARNING: missing player {entity_id:?}");
                }
            }
            // Ship Controls System
            for (_entity_id, _placement, controls, physics) in
                ecs_iter!(world, cur Placement, cur ShipControls, mut Physics)
            {
                physics.apply_force(controls.movement * 0.005);
            }
            // Physics System
            for (_entity_id, position, physics) in ecs_iter!(world, mut Placement, mut Physics) {
                let drag_amount = physics.velocity.magnitude_squared() * -0.5;
                if drag_amount != 0.0 {
                    physics.apply_force(physics.velocity.normalize() * drag_amount);
                }
                let linear_acceleration = physics.force / physics.mass;
                let angular_acceleration = physics.torque / physics.moment;
                position.position += physics.velocity + linear_acceleration * 0.5;
                position.angle += physics.angular_velocity + angular_acceleration * 0.5;
                position.angle = position.angle % TAU;
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
    pub fn with_ecs_world(&mut self, handler: impl FnOnce(&EcsWorld)) {
        self.ecs_world
            .with_origin(self.prev_ecs_world.clone(), |x| handler(x))
    }
}
