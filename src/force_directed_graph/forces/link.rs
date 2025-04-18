use crate::force_directed_graph::{
    common::{alpha, MouseLocked, NodeLink, NodePhysics},
    utils::ClampF32Range as _,
};
use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

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
            let direction = (target_position - source_position).clamp_f32_range();
            let distance = direction.length().clamp_f32_range();

            // prevent divide by zero and clamp to avoid too big forces
            let distance = distance.clamp(10.0, 1000.0);

            let strength = 1.0;
            let force = (((distance - link.target_distance).clamp_f32_range() / distance)
                .clamp_f32_range()
                * strength)
                .clamp_f32_range();

            ((direction * force).clamp_f32_range() * alpha(time.delta_secs())).clamp_f32_range()
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
    }
}
