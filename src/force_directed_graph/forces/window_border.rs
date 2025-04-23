use crate::force_directed_graph::{
    common::{MouseLocked, NodePhysics},
    utils::ClampF32Range,
};
use bevy::{
    ecs::system::Query,
    render::camera::{Camera, OrthographicProjection},
    transform::components::{GlobalTransform, Transform},
};

/// This is not really a force. It shifts all nodes inside the visible area of
/// the window.
///
/// * `bounce` - how much velocity is conserved when bouncing off the walls. 0.0
///   for no bounce whatsoever, 1.0 for full bounce.
pub fn apply_window_border(
    bounce: f32,
) -> impl Fn(
    Query<'_, '_, (&mut Transform, &mut NodePhysics, Option<&MouseLocked>)>,
    Query<'_, '_, (&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    move |mut transforms_q: Query<(&mut Transform, &mut NodePhysics, Option<&MouseLocked>)>,
          camera_q: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>| {
        let margin = 30.0;

        let (_camera, projection, transform) = camera_q.get_single().unwrap();
        let half_width = (projection.area.width() - margin).max(1.0) / 2.0;
        let half_height = (projection.area.height() - margin).max(1.0) / 2.0;
        let camera_x = transform.translation().x;
        let camera_y = transform.translation().y;
        let x_min = (camera_x - half_width).clamp_f32_range();
        let x_max = (camera_x + half_width).clamp_f32_range();
        let y_min = (camera_y - half_height).clamp_f32_range();
        let y_max = (camera_y + half_height).clamp_f32_range();

        for (mut transform, mut node_physics, _mouse_locked) in &mut transforms_q {
            //transform.translation.x = transform.translation.x.clamp(x_min, x_max);
            //transform.translation.y = transform.translation.y.clamp(y_min, y_max);
            //node_physics.previous_position.x = node_physics.previous_position.x.clamp(x_min, x_max);
            //node_physics.previous_position.y = node_physics.previous_position.y.clamp(y_min, y_max);

            let delta_position = transform.translation.truncate() - node_physics.previous_position;
            if transform.translation.x <= x_min || transform.translation.x >= x_max {
                // project node onto the wall
                transform.translation.x = transform.translation.x.clamp(x_min, x_max);
                // reverse velocity
                node_physics.previous_position.x =
                    transform.translation.x + delta_position.x * bounce;
            }
            if transform.translation.y <= y_min || transform.translation.y >= y_max {
                // project node onto the wall
                transform.translation.y = transform.translation.y.clamp(y_min, y_max);
                // reverse velocity
                node_physics.previous_position.y =
                    transform.translation.y + delta_position.y * bounce;
            }

            // This is for debugging only, if by a bug we end up with NaN in the transform
            #[cfg(debug_assertions)]
            assert!(transform.is_finite(), "Not finite: {:?}", transform);
        }
    }
}
