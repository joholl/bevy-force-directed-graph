use bevy::{
    ecs::{query::With, system::Query},
    math::Vec2,
    transform::components::Transform,
};

use crate::common::{MouseLocked, NodePhysics};

/* This is not really a force. It shifts all nodes so that their mean is in the middle. */
pub fn apply_center_force(
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
    }
}
