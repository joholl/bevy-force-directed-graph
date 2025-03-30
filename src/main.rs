use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::{prelude::*, window};
use common::{NodeLink, NodePhysics};
use rand::seq::IndexedRandom as _;
use rand::Rng;

mod common;
mod forces;
mod inertia;
mod mouse;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Force-directed Graph".to_string(),
                present_mode: window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 12.0,
                    ..default()
                },
                text_color: Color::WHITE.with_alpha(0.3),
                enabled: true,
            },
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                forces::mean_to_center::apply_mean_to_center,
                forces::link::apply_link_force,
                forces::repulsion::apply_repulsion_force,
                //forces::galaxy::apply_galaxy_force,
                forces::window_border::apply_window_border,
                inertia::apply_velocity,
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
    let num_entities: u16 = 300;
    let entities = (0..num_entities)
        .map(|i| {
            let radius = 10.0;
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
                source: *a,
                target: *b,
                target_distance: rng.random_range(30.0..100.0),
            },
            // will be transformed
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.5))),
            // will be updated later
            Transform::default(),
        ));
    }

    // commands.spawn((
    //     Text::new("Press space to toggle wireframes"),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     },
    // ));
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
