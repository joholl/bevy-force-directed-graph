use crate::force_directed_graph::{
    common::{MouseLocked, NodePhysics},
    utils::ClampF32Range as _,
    verlet::VerletRes,
};
use bevy::{
    ecs::system::{Query, Res},
    transform::components::Transform,
};

/// Add a constant friction force which counter-acts all movement.
/// * `strength` - friction force in kg*px/s^2
pub fn apply_friction(
    strength: f32,
) -> impl Fn(Query<'_, '_, (&mut Transform, &NodePhysics, Option<&MouseLocked>)>, Res<'_, VerletRes>)
{
    move |mut transforms_q: Query<(&mut Transform, &NodePhysics, Option<&MouseLocked>)>,
          verlet: Res<VerletRes>| {
        transforms_q
            .iter_mut()
            .filter(|(_, _, mouse_locked)| mouse_locked.is_none())
            .for_each(|(mut transform, node_physics, _)| {
                let movement = (transform.translation.truncate() - node_physics.previous_position)
                    .clamp_f32_range();
                if movement.length() == 0.0 {
                    return;
                }
                let movement_direction = (transform.translation.truncate()
                    - node_physics.previous_position)
                    .clamp_f32_range()
                    .normalize();
                let force = ((-movement_direction * strength).clamp_f32_range()
                    * verlet.delta_secs_squared())
                .clamp_f32_range();

                transform.translation =
                    (transform.translation + force.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(transform.is_finite(), "Not finite: {:?}", transform);
            });
    }
}
