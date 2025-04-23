use crate::force_directed_graph::{
    common::{MouseLocked, NodePhysics},
    utils::ClampF32Range as _,
    verlet::VerletRes,
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};

/// Apply an initial velocity to nodes that are not mouse locked.
/// * `velocity` - velocity in px/s (mass is irrelevant)
pub fn apply_initial_velocity(
    velocity: f32,
) -> impl Fn(
    Query<'_, '_, (&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    Res<'_, VerletRes>,
    Res<'_, Time>,
) {
    move |mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
          verlet: Res<VerletRes>,
          time: Res<Time>| {
        if time.elapsed_secs() == 0.0 {
            transforms_q
                .iter_mut()
                .filter(|(_, mouse_locked)| mouse_locked.is_none())
                .for_each(|(mut transform, _)| {
                    let nudge = (velocity * Vec2::NEG_Y * verlet.delta_secs()).clamp_f32_range();

                    transform.translation =
                        (transform.translation + nudge.extend(0.0)).clamp_f32_range();
                    #[cfg(debug_assertions)]
                    assert!(transform.is_finite(), "Not finite: {:?}", transform);
                });
        }
    }
}
