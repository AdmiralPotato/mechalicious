// PID != Process ID.
// PID != Pathways Into Darkness.
// PID == Proportion, Integral, Derivative

#[derive(Clone, Debug)]
pub struct PidController {
    proportional_coefficient: f32,
    integral_coefficient: f32,
    derivative_coefficient: f32,
    integral: f32,
}

impl PidController {
    pub const fn new(
        proportional_coefficient: f32,
        integral_coefficient: f32,
        derivative_coefficient: f32,
    ) -> PidController {
        PidController {
            proportional_coefficient,
            integral_coefficient,
            derivative_coefficient,
            integral: 0.0,
        }
    }
    pub fn get_control_output(&mut self, target_delta: f32, current_velocity: f32) -> f32 {
        let ret = target_delta * self.proportional_coefficient
            + (self.integral + target_delta * 0.5) * self.integral_coefficient
            - current_velocity * self.derivative_coefficient;
        self.integral += target_delta;
        ret.clamp(-1.0, 1.0)
    }
}

impl Default for PidController {
    fn default() -> Self {
        // By default, we're more like a PD controller.
        Self {
            proportional_coefficient: 1.0,
            integral_coefficient: 0.0,
            derivative_coefficient: 5.0,
            integral: 0.0,
        }
    }
}
