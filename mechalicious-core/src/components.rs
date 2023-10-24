use super::*;

#[derive(Clone, Debug, Default)]
pub struct Placement {
    pub position: Point,
    pub angle: f32,
    pub scale: f32,
}

impl Placement {
    pub fn as_similarity(&self) -> Similarity {
        Similarity::new(self.position.coords, self.angle, self.scale)
    }
    pub fn get_phased_transform(&self, prev_state: &Placement, phase: f32) -> Transform {
        let phased_position = prev_state.position.coords
            + (self.position.coords - prev_state.position.coords) * phase;
        let phased_angle = angle_lerp(prev_state.angle, self.angle, phase);
        //prev_state.angle + (self.angle - prev_state.angle) * phase;
        let similarity = Similarity::new(phased_position, phased_angle, self.scale);
        Transform::from_matrix_unchecked(similarity.to_homogeneous())
    }
    pub fn lerp_toward(&mut self, target: &Placement, theta: f32) {
        self.position = lerp(self.position.coords, target.position.coords, theta).into();
        self.scale = lerp(self.scale, target.scale, theta);
        self.angle = angle_lerp(self.angle, target.angle, theta);
    }
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

impl Physics {
    pub fn apply_force(&mut self, force: Vector) {
        self.force += force;
    }
    pub fn apply_torque(&mut self, torque: f32) {
        self.torque += torque;
    }
}

#[derive(Clone, Debug)]
pub struct Visible {
    pub model_path: &'static str,
}

#[derive(Clone, Debug, Default)]
pub struct ShipControls {
    pub movement: Vector, // left stick
    pub aim: Vector,      // right stick
    pub fire: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ShipControlCharacteristics {
    pub aim_controller: PidController,
}

#[derive(Clone, Debug)]
pub struct WorldPhysics {
    pub air_thickness: f32,
}
