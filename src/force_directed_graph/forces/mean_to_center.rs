use crate::force_directed_graph::common::{MouseLocked, NodePhysics};
use bevy::{
    ecs::{query::With, system::Query},
    math::Vec2,
    transform::components::Transform,
};

/// This is not really a force. It shifts all nodes so that their mean is in the
/// middle.
///
/// If one node is mouse locked, that node will not be shifted. This means that
/// one iteration is not enough to shift the overall mean exactly in the center.
/// With many nodes that is not significant. Also, later iterations will fix
/// that.
pub fn apply_mean_to_center(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
) {
    let center = Vec2::ZERO;

    // TODO iterates twice - unnecessary?
    // TODO panics when there is 0 nodes?
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

        #[cfg(debug_assertions)]
        assert!(transform.is_finite(), "Not finite: {:?}", transform);
    }
}

#[cfg(test)]
mod tests {
    use super::apply_mean_to_center;
    use crate::force_directed_graph::common::{MouseLocked, NodePhysics};
    use bevy::app::{App, Update};
    use bevy::math::Vec3;
    use bevy::transform::components::Transform;

    #[test]
    fn test_apply_mean_to_center() {
        let mut app = App::new();

        app.add_systems(Update, (apply_mean_to_center,));

        let transforms = [
            /* 2 | B
             * 1 C   A
             * --+-D---
             *   | 1 2
             */
            Vec3::new(2.0, 1.0, 10.0),
            Vec3::new(1.0, 2.0, 20.0),
            Vec3::new(0.0, 1.0, 30.0),
            Vec3::new(1.0, 0.0, 40.0),
        ]
        .map(Transform::from_translation);

        let entities = transforms.map(|t| {
            app.world_mut()
                .spawn((NodePhysics::from_transform(t), t))
                .id()
        });

        // Run systems
        app.update();

        let transforms_updated = entities.map(|id| *app.world().get::<Transform>(id).unwrap());
        let transforms_expected = [
            /*     2 |
             *     1 B
             *  ---C-+-A---
             *       D 1 2
             *       |
             */
            Vec3::new(1.0, 0.0, 10.0),
            Vec3::new(0.0, 1.0, 20.0),
            Vec3::new(-1.0, 0.0, 30.0),
            Vec3::new(0.0, -1.0, 40.0),
        ]
        .map(Transform::from_translation);

        assert_eq!(transforms_updated, transforms_expected);
    }

    #[test]
    fn test_apply_mean_to_center_mouse_locked() {
        let mut app = App::new();

        app.add_systems(Update, (apply_mean_to_center,));

        let transform1 = Transform::from_translation(Vec3::new(9.0, 8.0, 111.0));
        let transform2_mouse_locked = Transform::from_translation(Vec3::new(1.0, 2.0, 222.0));

        let entities = [
            app.world_mut()
                .spawn((NodePhysics::from_transform(transform1), transform1))
                .id(),
            app.world_mut()
                .spawn((
                    NodePhysics::from_transform(transform2_mouse_locked),
                    transform2_mouse_locked,
                    MouseLocked,
                ))
                .id(),
        ];

        // Run systems
        app.update();

        let transforms_updated = entities.map(|id| *app.world().get::<Transform>(id).unwrap());
        let transforms_expected = [
            Transform::from_translation(Vec3::new(4.0, 3.0, 111.0)),
            transform2_mouse_locked,
        ];
        assert_eq!(transforms_updated, transforms_expected);

        // Run systems
        app.update();

        let transforms_updated = entities.map(|id| *app.world().get::<Transform>(id).unwrap());
        let transforms_expected = [
            Transform::from_translation(Vec3::new(1.5, 0.5, 111.0)),
            transform2_mouse_locked,
        ];
        assert_eq!(transforms_updated, transforms_expected);
    }
}
