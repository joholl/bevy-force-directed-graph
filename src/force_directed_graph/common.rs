use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
    transform::components::Transform,
};

/// Component to save a nodes previous position. Instead of saving the velocity,
/// we approximate the velocity via the difference between the current and
/// previous position. This is needed for inertia.
#[derive(Component, Debug)]
pub struct NodePhysics {
    pub previous_position: Vec2,
}

impl NodePhysics {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            previous_position: transform.translation.truncate(),
        }
    }
}

/// Component for saving which nodes a given link connects.
#[derive(Component)]
pub struct NodeLink {
    pub source: Entity,
    pub target: Entity,
    pub target_distance: f32,
}

// `MouseLocked` is a marker component for node entities which are currently
// drag-and-dropped. This is needed for disabling forces and inertia
// temporarily. Additionally, we save the velocity of the mouse movement to
// apply it after the user lets go.
#[derive(Component, Debug)]
pub struct MouseLocked {
    pub velocity: Vec2,
}
