use super::*;

#[derive(Clone, Debug)]
pub struct Position {
    pub position: Point,
    pub angle: f32,
}

#[derive(Clone, Debug)]
pub struct Physics {
    pub mass: f32,
    pub moment: f32, // Solra will explain this later
    pub force: Vector,
    pub torque: f32,
    pub velocity: Vector,
    pub angular_velocity: f32,
}

#[derive(Clone, Debug)]
pub struct Visible {
    pub model_path: &'static str,
}

#[derive(Clone, Debug)]
pub struct ShipControls {
    pub movement: Vector, // left stick
    pub aim: Vector, // right stick
    pub fire: bool,
}
