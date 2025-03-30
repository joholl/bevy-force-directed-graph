use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

use crate::common::{alpha, MouseLocked, NodePhysics};

pub fn apply_repulsion_force(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    let mut combinations = transforms_q.iter_combinations_mut::<2>();
    while let Some([(mut a_transform, a_mouse_locked), (mut b_transform, b_mouse_locked)]) =
        combinations.fetch_next()
    {
        let direction = b_transform.translation.truncate() - a_transform.translation.truncate();
        let distance = direction.length();

        // Calculate the repulsion (minus!) based on the distance
        let strength = 100000.0;
        let force =
            -alpha(time.delta_secs()) * strength / (distance * distance) * direction.normalize();

        // Update the positions of both nodes
        if a_mouse_locked.is_none() {
            a_transform.translation += force.extend(0.0);
        }
        if b_mouse_locked.is_none() {
            b_transform.translation -= force.extend(0.0);
        }
    }
}
