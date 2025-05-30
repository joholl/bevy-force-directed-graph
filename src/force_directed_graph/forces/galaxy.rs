use crate::force_directed_graph::{
    common::{MouseLocked, NodePhysics},
    utils::{ClampF32Range as _, FiniteOr},
    verlet::VerletRes,
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::{Quat, Vec3},
    transform::components::Transform,
};

/// Add a force for counter-clockwise rotation around the center of the screen.
/// * `strength` - force in kg*px/s^2
pub fn apply_galaxy_force(
    strength: f32,
) -> impl Fn(Query<'_, '_, (&mut Transform, Option<&MouseLocked>), With<NodePhysics>>, Res<'_, VerletRes>)
{
    move |mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
          verlet: Res<VerletRes>| {
        transforms_q
            .iter_mut()
            .filter(|(_, mouse_locked)| mouse_locked.is_none())
            .for_each(|(mut transform, _)| {
                let position = transform.translation.truncate().extend(0.0);
                let position_rotated_by_90 = (Quat::from_rotation_z(90.0_f32.to_radians())
                    * position)
                    .clamp_f32_range()
                    .finite_or(Vec3::ZERO);
                let force = ((position_rotated_by_90 * strength).clamp_f32_range()
                    * verlet.delta_secs_squared())
                .clamp_f32_range();

                transform.translation = (transform.translation + force).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(transform.is_finite(), "Not finite: {:?}", transform);
            });
    }
}
