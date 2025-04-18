use crate::force_directed_graph::{
    common::{alpha, MouseLocked, NodePhysics},
    utils::{ClampF32Range, FiniteOrRandom as _},
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

pub fn apply_repulsion_force(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    let mut combinations = transforms_q.iter_combinations_mut::<2>();
    while let Some([(mut a_transform, a_mouse_locked), (mut b_transform, b_mouse_locked)]) =
        combinations.fetch_next()
    {
        let direction = (b_transform.translation.truncate() - a_transform.translation.truncate())
            .clamp_f32_range();
        let distance = direction.length().clamp_f32_range();

        // deal with NaN and zero
        // this also prevents overly big forces if nodes get too close
        let distance = distance.clamp(10.0, f32::MAX);

        // if the direction vector is zero, normalizing will lead to NaN (-> take a random direction)
        let direction = direction.normalize().finite_or_random_normalized();

        // Calculate the repulsion based on the distance
        let strength = 100000.0;
        let force = (((alpha(time.delta_secs()) * strength).clamp_f32_range()
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
