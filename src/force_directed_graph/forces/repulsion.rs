use crate::force_directed_graph::{
    common::{MouseLocked, NodePhysics},
    utils::{ClampF32Range, FiniteOrRandom as _},
    verlet::VerletRes,
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    transform::components::Transform,
};

/// Add a repulsion force.
/// * `strength` - force in kg*px/s^2; decreses with distance (1 / r^2)
pub fn apply_repulsion_force(
    strength: f32,
) -> impl Fn(
    Query<'_, '_, (&mut Transform, std::option::Option<&MouseLocked>), With<NodePhysics>>,
    Res<'_, VerletRes>,
) {
    move |mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
          verlet: Res<VerletRes>| {
        let mut combinations = transforms_q.iter_combinations_mut::<2>();
        while let Some([(mut a_transform, a_mouse_locked), (mut b_transform, b_mouse_locked)]) =
            combinations.fetch_next()
        {
            let direction = (b_transform.translation.truncate()
                - a_transform.translation.truncate())
            .clamp_f32_range();
            let distance = direction.length().clamp_f32_range();

            // deal with NaN and zero
            // this also prevents overly big forces if nodes get too close
            let distance = distance.clamp(10.0, f32::MAX);

            // if the direction vector is zero, normalizing will lead to NaN (-> take a random direction)
            let direction = direction.normalize().finite_or_random_normalized();

            // Calculate the repulsion based on the distance
            let force = (((verlet.delta_secs_squared() * strength).clamp_f32_range()
                / (distance * distance).clamp_f32_range())
            .clamp_f32_range()
                * direction)
                .clamp_f32_range();

            // Update the positions of both nodes
            if a_mouse_locked.is_none() {
                a_transform.translation =
                    (a_transform.translation - force.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(a_transform.is_finite(), "Not finite: {:?}", a_transform);
            }
            if b_mouse_locked.is_none() {
                b_transform.translation =
                    (b_transform.translation + force.extend(0.0)).clamp_f32_range();
                #[cfg(debug_assertions)]
                assert!(b_transform.is_finite(), "Not finite: {:?}", b_transform);
            }
        }
    }
}
