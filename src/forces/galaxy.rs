use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::Quat,
    time::Time,
    transform::components::Transform,
};

use crate::common::{alpha, MouseLocked, NodePhysics};

/// Add a force for counter-clockwise rotation around the center of the screen.
pub fn apply_galaxy_force(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    for (mut transform, mouse_locked) in &mut transforms_q {
        if mouse_locked.is_none() {
            let position = transform.translation;
            let position_rotated_by_90 = Quat::from_rotation_z(90.0_f32.to_radians()) * position;
            let strength = 0.03;
            let force = position_rotated_by_90 * strength * alpha(time.delta_secs());

            if mouse_locked.is_none() {
                transform.translation += force;
            }
        }
    }
}
