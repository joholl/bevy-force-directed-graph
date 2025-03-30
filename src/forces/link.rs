use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

use crate::common::{alpha, MouseLocked, NodeLink, NodePhysics};

pub fn apply_link_force(
    links_q: Query<&NodeLink, Without<NodePhysics>>,
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    for link in &links_q {
        let position_delta = {
            let (source_transform, _) = transforms_q.get(link.source).unwrap();
            let (target_transform, _) = transforms_q.get(link.target).unwrap();

            let source_position = source_transform.translation.truncate();
            let target_position = target_transform.translation.truncate();

            // Calculate the direction and distance between the two nodes
            let direction = target_position - source_position;
            let distance = direction.length();

            let strength = 1.0;
            let force = (distance - link.target_distance) / distance * strength;

            direction * force * alpha(time.delta_secs())
        };

        let (mut source_transform, mouse_locked) = transforms_q.get_mut(link.source).unwrap();
        if mouse_locked.is_none() {
            source_transform.translation += position_delta.extend(0.0);
        }
        #[cfg(debug_assertions)]
        if source_transform.translation.x.is_nan()
            || source_transform.translation.y.is_nan()
            || source_transform.translation.z.is_nan()
        {
            panic!("NaN in transform: {:?}", &source_transform);
        }

        let (mut target_transform, mouse_locked) = transforms_q.get_mut(link.target).unwrap();
        if mouse_locked.is_none() {
            target_transform.translation -= position_delta.extend(0.0);
        }
        #[cfg(debug_assertions)]
        if target_transform.translation.x.is_nan()
            || target_transform.translation.y.is_nan()
            || target_transform.translation.z.is_nan()
        {
            panic!("NaN in transform: {:?}", &target_transform);
        }
    }
}
