use super::*;



#[derive(Clone, Debug)]
pub struct Placement {
    pub position: Point,
    pub angle: f32,
    pub scale: f32,
}

impl Placement {
    pub fn to_transform(&self) -> Transform {
        let similarity = Similarity::new(self.position.coords, self.angle, self.scale);
        Transform::from_matrix_unchecked(similarity.to_homogeneous())
    }
    pub fn to_phased_transform(&self, prev_state: &Placement, phase: f32) -> Transform {
        let phased_position = prev_state.position.coords
            + (self.position.coords - prev_state.position.coords) * phase;
        let phased_angle = angle_lerp(prev_state.angle, self.angle, phase);
        //prev_state.angle + (self.angle - prev_state.angle) * phase;
        let similarity = Similarity::new(phased_position, phased_angle, self.scale);
        Transform::from_matrix_unchecked(similarity.to_homogeneous())
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

#[derive(Clone, Debug)]
pub struct Visible {
    pub model_path: &'static str,
}

#[derive(Clone, Debug)]
pub struct ShipControls {
    pub movement: Vector, // left stick
    pub aim: Vector,      // right stick
    pub fire: bool,
}
