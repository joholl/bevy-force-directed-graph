use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Component)]
pub struct NodeIndex(pub petgraph::prelude::NodeIndex);

#[derive(Component)]
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
#[derive(Component)]
pub struct NodeLink {
    pub source: Entity,
    pub target: Entity,
    pub target_distance: f32,
}

#[derive(Component, Debug)]
pub struct MouseLocked;

/* Account for variable frame rate (i.e. variable time step) */
pub fn alpha(dt_secs: f32) -> f32 {
    2.4 * dt_secs
}
