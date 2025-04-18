use crate::force_directed_graph::{
    common::{alpha, MouseLocked, NodePhysics},
    utils::{ClampF32Range as _, FiniteOr},
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::{Quat, Vec3},
    time::Time,
    transform::components::Transform,
};

/// Add a force for counter-clockwise rotation around the center of the screen.
pub fn apply_galaxy_force(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    for (mut transform, mouse_locked) in &mut transforms_q {
        if mouse_locked.is_none() {
            let position = transform.translation.truncate().extend(0.0);
            let position_rotated_by_90 = (Quat::from_rotation_z(90.0_f32.to_radians()) * position)
                .clamp_f32_range()
                .finite_or(Vec3::ZERO);
            let strength = 0.03;
            let force = position_rotated_by_90 * strength * alpha(time.delta_secs());

            transform.translation = (transform.translation + force).clamp_f32_range();
            #[cfg(debug_assertions)]
            assert!(transform.is_finite(), "Not finite: {:?}", transform);
        }
    }
}
