use bevy::{
    ecs::{
        observer::Trigger,
        system::{Commands, Query},
    },
    math::{Vec2, Vec3},
    picking::events::{Drag, DragEnd, DragStart, Pointer},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
};

use crate::force_directed_graph::{common::MouseLocked, utils::ClampF32Range};

use super::common::NodePhysics;

/// Observer for drag-and-drop events. Requires a sprite for now. Moves the
/// entity (node) to the mouse position.
pub fn drag_n_drop(
    trigger: Trigger<Pointer<Drag>>,
    mut transforms_q: Query<&mut Transform>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let mut transform = transforms_q.get_mut(trigger.entity()).unwrap();

    let (camera, camera_transform) = camera_q.get_single().expect("Expected a single camera");
    let world_pos = camera
        .viewport_to_world_2d(camera_transform, trigger.event().pointer_location.position)
        .expect("Camera's projection matrix is invalid");
    transform.translation = Vec3::new(world_pos.x, world_pos.y, transform.translation.z);
}

/// Observer for drag-and-drop events. Adds a `MouseLocked` component to the
/// node entity. This is needed for disabling forces and inertia.
pub fn drag_start(trigger: Trigger<Pointer<DragStart>>, mut commands: Commands) {
    commands.entity(trigger.event().target).insert(MouseLocked {
        velocity: Vec2::ZERO,
    });
}

/// Observer for drag-and-drop events. Removes the `MouseLocked` component from
/// the node entity. See [drag_start].
pub fn drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    mut transforms_q: Query<(&mut NodePhysics, &MouseLocked)>,
    mut commands: Commands,
) {
    let (mut node_physics, mouse_locked) = transforms_q.get_mut(trigger.entity()).unwrap();

    // since verlet integration was locked, position is equal to previous position
    // manipulate previous position to achive mouse velocity
    node_physics.previous_position =
        (node_physics.previous_position - mouse_locked.velocity).clamp_f32_range();

    commands
        .entity(trigger.event().target)
        .remove::<MouseLocked>();
}
