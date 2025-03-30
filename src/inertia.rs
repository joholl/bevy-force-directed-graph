use bevy::{
    ecs::system::{Query, Res},
    time::Time,
    transform::components::Transform,
};

use crate::common::{MouseLocked, NodePhysics};

/// Given a node position (Transform), its previous position, and acceleration (NodePhysics), this function
/// calculates the new position using Verlet integration.
///
/// Usually, the Verlet integration considers the acceleration (based on a mass of 1).
/// However, here the forces change the current position directly, so we do not need to consider the acceleration.
///
/// Inputs:
///  - node position: result of last Verlet integration step
///  - previous position: result of the penultimate Verlet integration step
///
/// Outputs:
///  - node position: new position of the node
///  - previous position: result of the last Verlet integration step
pub fn apply_velocity(
    mut nodes_q: Query<(&mut Transform, &mut NodePhysics, Option<&MouseLocked>)>,
    _time: Res<Time>,
) {
    // TODO when dt changes, this gets messy
    // e.g.
    //  * dt_previous = 1s
    //    dx_previous = pos_penultimate - pos_previous = 1m
    //  * dt_current = 0.5s
    //    dx_current = pos_previous - pos_current =
    //    => speedup!?

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
        }
        node_physics.previous_position = previous_position_next;

        #[cfg(debug_assertions)]
        if transform.translation.x.is_nan()
            || transform.translation.y.is_nan()
            || transform.translation.z.is_nan()
        {
            panic!("NaN in transform: {:?}", &transform);
        }
    }
}
