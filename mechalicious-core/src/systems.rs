use super::*;

impl GameWorld {
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
            for (_entity_id, placement, controls, control_characteristics, physics) in
                ecs_iter!(world, mut Placement, cur ShipControls, mut ShipControlCharacteristics, mut Physics)
            {
                physics.apply_force(controls.movement * 0.005);
                let facing_angle = placement.angle;
                let aim_angle = controls.aim.y.atan2(controls.aim.x);
                let diff = angle_subtract(aim_angle, facing_angle);
                physics.apply_torque(control_characteristics.aim_controller.get_control_output(diff, physics.angular_velocity) * 0.05);
            }
            // Physics System
            let world_physics = ecs_singleton!(world, cur WorldPhysics);
            for (_entity_id, position, physics) in ecs_iter!(world, mut Placement, mut Physics) {
                let drag_amount =
                    physics.velocity.magnitude_squared() * -world_physics.air_thickness;
                if drag_amount != 0.0 {
                    physics.apply_force(physics.velocity.normalize() * drag_amount);
                }
                let linear_acceleration = physics.force / physics.mass;
                let angular_acceleration = physics.torque / physics.moment;
                position.position += physics.velocity + linear_acceleration * 0.5;
                position.angle += physics.angular_velocity + angular_acceleration * 0.5;
                position.angle %= TAU;
                physics.velocity += linear_acceleration;
                physics.angular_velocity += angular_acceleration;
                // zero out the forces for next tick
                physics.force = vector![0.0, 0.0];
                physics.torque = 0.0;
            }
        });
    }
}
