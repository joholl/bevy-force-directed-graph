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
    transform::components::Transform,
};

/// Add a gravity force.
pub fn apply_gravity_force(
    strength: f32,
) -> impl Fn(Query<'_, '_, (&mut Transform, Option<&MouseLocked>), With<NodePhysics>>, Res<'_, VerletRes>)
{
    move |mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
          verlet: Res<VerletRes>| {
        transforms_q
            .iter_mut()
            .filter(|(_, mouse_locked)| mouse_locked.is_none())
            .for_each(|(mut transform, _)| {
                let force =
                    (strength * Vec2::NEG_Y * verlet.delta_secs_squared()).clamp_f32_range();

                transform.translation =
                    (transform.translation + force.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(transform.is_finite(), "Not finite: {:?}", transform);
            });
    }
}
