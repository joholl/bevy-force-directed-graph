use bevy::{
    ecs::{
        observer::Trigger,
        system::{Commands, Query},
    },
    math::Vec3,
    picking::events::{Drag, DragEnd, DragStart, Pointer},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
};

use crate::bevy::common::MouseLocked;

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

pub fn drag_start(trigger: Trigger<Pointer<DragStart>>, mut commands: Commands) {
    commands.entity(trigger.event().target).insert(MouseLocked);
}

pub fn drag_end(trigger: Trigger<Pointer<DragEnd>>, mut commands: Commands) {
    commands
        .entity(trigger.event().target)
        .remove::<MouseLocked>();
}
