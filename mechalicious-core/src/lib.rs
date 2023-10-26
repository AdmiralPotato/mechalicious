use std::{
    f32::consts::{PI, TAU},
    ops::{Add, Mul},
};

use psilo_ecs::*;

use arcow::Arcow;
use nalgebra::{point, vector};
pub type Point = nalgebra::Point2<f32>;
pub type Vector = nalgebra::Vector2<f32>;
pub type Transform = nalgebra::Transform2<f32>;
pub type Similarity = nalgebra::Similarity2<f32>;
pub type Affine = nalgebra::Affine2<f32>;
pub type Scale = nalgebra::Scale2<f32>;
pub type Translation = nalgebra::Translation2<f32>;
/*

Note for future Admiral: If you find yourself needing to turn a Point into a
Vector, (for instance, when averaging points) use my_point.coords instead of
my_point.into(). my_point.coords will just give you the coordinates, as a vector. my_point.into() will convert it into a *homogenous* vector, `x, y, 1`.

*/

use rand::prelude::*;

// if we do: pub mod components
// then our dependencies can use: mechalicious_core::components::Position
// but if we do: pub use components::*
// then our dependencies can also use: mechalicious_core::Position

pub mod pid;
use pid::*;

pub mod components;
use components::*;

mod systems;

pub fn angle_subtract(a: f32, b: f32) -> f32 {
    let delta = a - b;
    if delta.abs() >= PI {
        if delta > 0.0 {
            delta - TAU
        } else {
            delta + TAU
        }
    } else {
        delta
    }
}

pub fn angle_lerp(a: f32, b: f32, theta: f32) -> f32 {
    let delta = angle_subtract(b, a);
    a + (delta * theta)
}

pub fn lerp<T: Mul<f32, Output = T> + Add<T, Output = T>>(a: T, b: T, theta: f32) -> T {
    (a * (1.0 - theta)) + (b * theta)
}

pub struct GameWorld {
    prev_ecs_world: Arcow<EcsWorld>,
    ecs_world: Arcow<EcsWorld>,
}

impl GameWorld {
    pub fn new_test_world() -> GameWorld {
        let mut ecs_world = EcsWorld::with_blank_schema();
        ecs_spawn!(ecs_world, WorldPhysics { air_thickness: 5.4 },);
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
                fire: false,
            },
            ShipControlCharacteristics::default(),
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
                angular_velocity: 0.0,
            },
            ShipControls {
                movement: vector![0.0, 0.0],
                aim: vector![0.0, 0.0],
                fire: false,
            },
            ShipControlCharacteristics::default(),
            Visible {
                model_path: "mechalicious.v2d",
            },
        );
        let mut thread_rng = thread_rng();
        for _ in 0..600 {
            let x = thread_rng.gen_range(-10.0..10.0);
            let y = thread_rng.gen_range(-10.0..10.0);
            ecs_spawn!(
                ecs_world,
                Placement {
                    position: point![x, y],
                    angle: thread_rng.gen_range(0.0..TAU),
                    scale: 0.1,
                },
                Visible {
                    model_path: "mechalicious.v2d",
                },
            );
        }
        let ecs_world = Arcow::new(ecs_world);
        GameWorld {
            prev_ecs_world: ecs_world.clone(),
            ecs_world,
        }
    }
    pub fn get_ecs_world(&self) -> &EcsWorld {
        &self.ecs_world
    }
    pub fn with_ecs_world(&mut self, handler: impl FnOnce(&EcsWorld)) {
        self.ecs_world
            .with_origin(self.prev_ecs_world.clone(), |x| handler(x))
    }
}

pub fn similarity_to_transform(similarity: Similarity) -> Transform {
    Transform::from_matrix_unchecked(similarity.to_homogeneous())
}

pub fn similarity_to_affine(similarity: Similarity) -> Affine {
    Affine::from_matrix_unchecked(similarity.to_homogeneous())
}

pub fn scale_to_affine(scale: Scale) -> Affine {
    Affine::from_matrix_unchecked(scale.to_homogeneous())
}

pub fn affine_to_transform(affine: Affine) -> Transform {
    Transform::from_matrix_unchecked(affine.to_homogeneous())
}
