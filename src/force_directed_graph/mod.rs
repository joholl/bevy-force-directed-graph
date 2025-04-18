use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::color::{Alpha as _, Color};
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::ecs::query::{With, Without};
use bevy::ecs::system::{Commands, Query};
use bevy::math::primitives::{Circle, Rectangle};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::PluginGroup;
use bevy::prelude::ResMut;
use bevy::render::mesh::{Mesh, Mesh2d};
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use bevy::text::TextFont;
use bevy::transform::components::Transform;
use bevy::utils::default;
use bevy::window::{self, Window, WindowPlugin};
use bevy::DefaultPlugins;
use common::{NodeLink, NodePhysics};
use rand::rngs::SmallRng;
use rand::seq::IndexedRandom as _;
use rand::{Rng as _, SeedableRng as _};

pub mod common;
pub mod forces;
pub mod inertia;
pub mod mouse;
pub mod utils;

/// Run the bevy application. Blocks until the window is closed.
pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                // WindowPlugin is needed to uncap framerate
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Force-directed Graph".to_string(),
                        present_mode: window::PresentMode::AutoNoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            // For drag-and-drop events
            MeshPickingPlugin,
            // FpsOverlayPlugin is needed to show framerate
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    text_color: Color::WHITE.with_alpha(0.3),
                    enabled: true,
                },
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                forces::mean_to_center::apply_mean_to_center,
                forces::link::apply_link_force,
                forces::repulsion::apply_repulsion_force,
                forces::galaxy::apply_galaxy_force,
                forces::window_border::apply_window_border,
                inertia::apply_velocity,
                update_links,
            ),
        )
        .run();
}

/// Spawn camera, nodes, and links
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = SmallRng::seed_from_u64(0);

    commands.spawn(Camera2d);

    // For every node ("entity")
    let num_entities: u16 = 50;
    let entities = (0..num_entities)
        .map(|i| {
            let radius = 15.0;
            let shape = meshes.add(Circle::new(radius));

            // Distribute colors evenly across the rainbow.
            let color = Color::hsl(360. * i as f32 / num_entities as f32, 0.95, 0.7);

            // Start position ("transform") in the center (but start slightly random)
            let transform = Transform::from_xyz(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(0.0..1.0),
            );

            // Spawn the node entity with its components (Sprite, Mesh2d, etc.)
            commands
                .spawn((
                    // Actual appearance
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    // X/Y position
                    transform,
                    // Additional physics information: previous position to approximate velocity for inertia
                    NodePhysics::from_transform(transform),
                ))
                .observe(mouse::drag_n_drop)
                .observe(mouse::drag_start)
                .observe(mouse::drag_end)
                .id()
        })
        .collect::<Vec<_>>();

    // Create random links between nodes
    for _ in 0..(f32::from(num_entities) * 1.2) as u32 {
        let a = entities.choose(&mut rng).unwrap();
        let b = entities.choose(&mut rng).unwrap();
        if a == b {
            continue;
        }

        commands.spawn((
            NodeLink {
                // The two nodes to be linked
                source: *a,
                target: *b,
                // Target distance for the link force
                target_distance: rng.random_range(50.0..150.0),
            },
            // Rectangle dimensions will be transformed later in [update_links]
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.5))),
            // Position will be transformed later [uodate_links]
            Transform::default(),
        ));
    }
}

/// Update the links between nodes (position and rotation)
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

        let thickness = 2.5;

        // Update the link's transform to match the source and target positions
        link_transform.translation = midpoint.extend(0.0);
        link_transform.rotation = Quat::from_rotation_z(angle);
        link_transform.scale = Vec3::new(length, thickness, 1.0);
    }
}
