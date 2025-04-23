use crate::force_directed_graph::{
    common::{MouseLocked, NodeLink, NodePhysics},
    utils::ClampF32Range as _,
    verlet::VerletRes,
};
use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    transform::components::Transform,
};

/// Add a spring force between two nodes. Equation: F = strength * (distance -
///   target_distance) / distance = strength * (1 - target_distance/direction)
///
/// E.g. if spring is extended 2x the target_distance, the force is equal to the
/// strength.
///
/// To avoid adding too much force, the distance is clamped to a minimum and
/// maximum value before calculating the force.
///
/// * `strength` - spring force in kg*px/s^2. If the nodes are 2x the target
///   distance apart, this is the amount of force that pulls them back together.
///   If the nodes are 0.5x the target distance together, this is the amount of
///   force that pushes them apart.
/// * `strength_min` - minimum (absolute) force acting on the two nodes (e.g. 0.0)
/// * `strength_max` - maximum (absolute) force acting on the two nodes (e.g.
///   f32::MAX)
///
pub fn apply_link_force(
    strength: f32,
    strength_max: f32,
) -> impl Fn(
    Query<'_, '_, &NodeLink, Without<NodePhysics>>,
    Query<'_, '_, (&mut Transform, std::option::Option<&MouseLocked>), With<NodePhysics>>,
    Res<'_, VerletRes>,
) {
    move |links_q: Query<&NodeLink, Without<NodePhysics>>,
          mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
          verlet: Res<VerletRes>| {
        links_q.iter().for_each(|link| {
            let position_delta = {
                let (source_transform, _) = transforms_q.get(link.source).unwrap();
                let (target_transform, _) = transforms_q.get(link.target).unwrap();

                let source_position = source_transform.translation.truncate();
                let target_position = target_transform.translation.truncate();

                // Calculate the direction and distance between the two nodes
                let direction = (target_position - source_position).clamp_f32_range();
                let distance = direction.length().clamp_f32_range();

                // prevent divide by zero and clamp to avoid too big forces
                let delta_distance = (distance - link.target_distance).clamp_f32_range();

                let force_abs = ((delta_distance.abs() / distance).clamp_f32_range() * strength)
                    .min(strength_max);
                let force_sign = delta_distance.signum();
                let force = force_sign * force_abs;

                ((direction * force).clamp_f32_range() * verlet.delta_secs_squared())
                    .clamp_f32_range()
            };

            let (mut source_transform, mouse_locked) = transforms_q.get_mut(link.source).unwrap();
            if mouse_locked.is_none() {
                source_transform.translation =
                    (source_transform.translation + position_delta.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(
                    source_transform.is_finite(),
                    "Not finite: {:?}",
                    source_transform
                );
            }

            let (mut target_transform, mouse_locked) = transforms_q.get_mut(link.target).unwrap();
            if mouse_locked.is_none() {
                target_transform.translation =
                    (target_transform.translation - position_delta.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(
                    target_transform.is_finite(),
                    "Not finite: {:?}",
                    target_transform
                );
            }
        });
    }
}
