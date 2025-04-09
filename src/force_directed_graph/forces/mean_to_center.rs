use crate::force_directed_graph::common::{MouseLocked, NodePhysics};
use bevy::{
    ecs::{query::With, system::Query},
    math::Vec2,
    transform::components::Transform,
};

/* This is not really a force. It shifts all nodes so that their mean is in the middle. */
pub fn apply_mean_to_center(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
) {
    let center = Vec2::ZERO;

    let mean = transforms_q
        .iter()
        .map(|(t, _)| t.translation.truncate())
        .sum::<Vec2>()
        / transforms_q.iter().count() as f32;

    let correction = center - mean;

    for (mut transform, mouse_locked) in &mut transforms_q {
        if mouse_locked.is_none() {
            transform.translation += correction.extend(0.0);
        }

        // This is for debugging only, if by a bug we end up with NaN in the transform
        #[cfg(debug_assertions)]
        if transform.translation.x.is_nan()
            || transform.translation.y.is_nan()
            || transform.translation.z.is_nan()
        {
            panic!("NaN in transform: {:?}", &transform);
        }
    }
}
