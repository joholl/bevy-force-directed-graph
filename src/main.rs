//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use std::iter;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Component)]
struct MyNode {
    pos: Vec2,
    size: f32,
}

impl MyNode {
    fn random(rand: &mut SmallRng) -> Self {
        MyNode {
            pos: Vec2::new(
                (rand.random::<f32>() - 0.5) * 1200.0, // TODO dimension hardcoded
                (rand.random::<f32>() - 0.5) * 600.0,  // TODO dimension hardcoded
            ),
            size: 4.0 + rand.random::<f32>() * 16.0, // TODO size hardcoded
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup);
    app.add_systems(Update, toggle_wireframe);
    app.add_systems(Update, animate);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes: Vec<Handle<Mesh>> =
        iter::from_fn(|| Some(meshes.add(RegularPolygon::new(50.0, 6))))
            .take(10)
            .collect();

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
        ));
    }

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

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut Mesh2d,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut Transform,
    )>,
) {
    let t = (ops::sin(time.elapsed_secs()) + 1.) / 2.;

    for (mut mesh, mut material, mut transform) in &mut query {
        transform.translation.y = 600. * (t - 0.5);
    }
}
