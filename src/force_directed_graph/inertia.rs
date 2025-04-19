use crate::force_directed_graph::common::{MouseLocked, NodePhysics};
use bevy::{
    ecs::system::{Query, Res},
    time::Time,
    transform::components::Transform,
};

use super::previous_time_delta::PreviousTimeDeltaSecs;

/// Given a node position (Transform) and its previous position (NodePhysics),
/// this function calculates the new position using Verlet integration.
///
/// Usually, the Verlet integration considers the acceleration (based on some
/// forces and a mass of 1). However, here the forces change the current
/// position directly, so we do not need to consider (i.e. save and calculate)
/// the acceleration.
///
/// Inputs:
///  - node position: result of last Verlet integration step
///  - previous position: result of the Verlet integration step before that
///
/// Outputs:
///  - node position: new position of the node
///  - previous position: result of the last Verlet integration step
pub fn apply_velocity(
    mut nodes_q: Query<(&mut Transform, &mut NodePhysics, Option<&MouseLocked>)>,
    time: Res<Time>,
    time_previous: Res<PreviousTimeDeltaSecs>,
) {
    // We need to account for non-constant time steps
    // https://en.wikipedia.org/wiki/Verlet_integration#Non-constant_time_differences
    let dt = time.delta_secs();
    let dt_previous = time_previous.delta_secs();

    //let dt = time.delta_secs();
    let velocity_decay = 0.95;

    // for node position (Transform), its previous position, and acceleration (NodePhysics)
    for (mut transform, mut node_physics, mouse_locked) in &mut nodes_q {
        // Current state
        let position = transform.translation.truncate();
        let previous_position = node_physics.previous_position;

        // Verlet integration step:
        // we do not store the velocity, but we can approximate it from the previous and current position
        // with the assumption that dt is equidistant
        let position_next = position + (position - previous_position) * velocity_decay;
        let previous_position_next = position;

        // Update state
        if mouse_locked.is_none() {
            transform.translation = position_next.extend(transform.translation.z);
            #[cfg(debug_assertions)]
            assert!(transform.is_finite(), "Not finite: {:?}", transform);
        }
        node_physics.previous_position = previous_position_next;
    }
}
