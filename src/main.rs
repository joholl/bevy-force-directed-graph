//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use std::iter;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse;
use bevy::math::VectorSpace;
use bevy::render::{alpha, camera};
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::utils::dbg;
use bevy::{prelude::*, transform};
use rand::seq::IndexedRandom as _;
use rand::Rng;

#[derive(Component)]
struct NodePhysics {
    previous_position: Vec2,
}

impl NodePhysics {
    fn from_transform(transform: Transform) -> Self {
        Self {
            previous_position: transform.translation.truncate(),
        }
    }
}

#[derive(Component)]
struct MouseLocked;

#[derive(Component)]
struct NodeLink {
    source: Entity,
    target: Entity,
    rest_length: f32,
}

fn main() {
    App::new()
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_center_force,
                apply_link_force,
                verlet_integration,
                update_links,
            ),
        )
        .run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();

    commands.spawn(Camera2d);

    //for (i, shape) in shapes.into_iter().enumerate() {
    let num_entities = 15;
    let entities = (0..num_entities)
        .map(|i| {
            let radius = 20.0;
            let shape = meshes.add(RegularPolygon::new(radius, 6));

            // Distribute colors evenly across the rainbow.
            let color = Color::hsl(360. * i as f32 / num_entities as f32, 0.95, 0.7);

            // Random transform (from -X_EXTENT/2 to X_EXTENT/2)
            let transform = Transform::from_xyz(
                rng.random_range(-X_EXTENT / 2.0..X_EXTENT / 2.0),
                rng.random_range(-X_EXTENT / 2.0..X_EXTENT / 2.0),
                rng.random_range(0.0..1.0),
            );

            commands
                .spawn((
                    Sprite {
                        // needed for drag and drop
                        color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(radius, radius)),
                        ..default()
                    },
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    transform,
                    NodePhysics::from_transform(transform),
                ))
                .observe(drag_n_drop)
                .observe(drag_start)
                .observe(drag_end)
                .id()
        })
        .collect::<Vec<_>>();

    // Create random links between nodes
    for _ in 0..25 {
        let a = entities.choose(&mut rng).unwrap();
        let b = entities.choose(&mut rng).unwrap();
        if a == b {
            continue;
        }

        commands.spawn((
            NodeLink {
                source: *a,
                target: *b,
                rest_length: 100.0,
            },
            // will be transformed
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.5))),
            // will be updated later
            Transform::default(),
        ));
    }

    // for entity in entities.iter() {
    //     if let links = commands.entity(*entity).queue() {
    //         for link in entities.iter_mut() {
    //             let target_transform = commands.entity(link.target).get::<Transform>().unwrap();
    //             let target_position = target_transform.translation.truncate();

    //             // Draw a line between the two nodes
    //             commands.spawn((
    //                 Mesh2d(meshes.add(Line::new(Vec2::ZERO, target_position - Vec2::ZERO))),
    //                 MeshMaterial2d(materials.add(Color::WHITE)),
    //                 Transform::from_translation(Vec3::ZERO),
    //             ));
    //         }
    //     }
    // }

    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn drag_n_drop(
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

fn drag_start(trigger: Trigger<Pointer<DragStart>>, mut commands: Commands) {
    commands.entity(trigger.event().target).insert(MouseLocked);
}

fn drag_end(trigger: Trigger<Pointer<DragEnd>>, mut commands: Commands) {
    commands
        .entity(trigger.event().target)
        .remove::<MouseLocked>();
}

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
fn verlet_integration(
    mut nodes_q: Query<(&mut Transform, &mut NodePhysics, Option<&MouseLocked>)>,
    _time: Res<Time>,
) {
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
    }
}

/* This is not really a force. It shifts all nodes so that their mean is in the middle. */
fn apply_center_force(mut transforms_q: Query<&mut Transform, With<NodePhysics>>) {
    let center = Vec2::ZERO;

    let mean = transforms_q
        .iter()
        .map(|t| t.translation.truncate())
        .sum::<Vec2>()
        / transforms_q.iter().count() as f32;

    let correction = center - mean;

    for mut transform in &mut transforms_q {
        transform.translation += correction.extend(0.0);
    }
}

fn update_links(
    mut links_q: Query<(&NodeLink, &mut Transform), Without<NodePhysics>>,
    transforms_q: Query<&Transform, With<NodePhysics>>,
) {
    for (link, mut link_transform) in &mut links_q {
        let source_transform = transforms_q.get(link.source).unwrap();
        let target_transform = transforms_q.get(link.target).unwrap();

        // Update the link's transform to match the source and target positions
        let source_position = source_transform.translation.truncate();
        let target_position = target_transform.translation.truncate();

        let midpoint = (source_position + target_position) / 2.0;
        let direction = target_position - source_position;
        let angle = Vec2::X.angle_to(direction);
        let length = direction.length();

        let thickness = 3.0;

        // Update the link's transform to match the source and target positions
        link_transform.translation = midpoint.extend(0.0);
        link_transform.rotation = Quat::from_rotation_z(angle);
        link_transform.scale = Vec3::new(length, thickness, 1.0);
    }
}

fn apply_link_force(
    links_q: Query<&NodeLink, Without<NodePhysics>>,
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
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

            let alpha = 0.04;
            let strength = 1.0;
            let force: f32 = (distance - 300.0) / distance * alpha * strength;

            direction * force
        };

        let (mut source_transform, mouse_locked) = transforms_q.get_mut(link.source).unwrap();
        if mouse_locked.is_none() {
            source_transform.translation += position_delta.extend(0.0);
        }
        let (mut target_transform, mouse_locked) = transforms_q.get_mut(link.target).unwrap();
        if mouse_locked.is_none() {
            target_transform.translation -= position_delta.extend(0.0);
        }
    }
}
