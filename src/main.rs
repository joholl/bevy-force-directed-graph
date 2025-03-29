//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use std::iter;

use avian2d::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use rand::rngs::SmallRng;
use rand::Rng;

// #[derive(Component)]
// struct MyNode {
//     pos: Vec2,
//     size: f32,
// }

// impl MyNode {
//     fn random(rand: &mut SmallRng) -> Self {
//         MyNode {
//             pos: Vec2::new(
//                 (rand.random::<f32>() - 0.5) * 1200.0, // TODO dimension hardcoded
//                 (rand.random::<f32>() - 0.5) * 600.0,  // TODO dimension hardcoded
//             ),
//             size: 4.0 + rand.random::<f32>() * 16.0, // TODO size hardcoded
//         }
//     }
// }

fn main() {
    App::new()
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(PhysicsPickingPlugin)
        //.add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        //.add_systems(Update, toggle_wireframe)
        //.add_systems(Update, animate)
        .insert_resource(Gravity::ZERO)
        .run();
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

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

    //     commands.spawn((
    //         Mesh2d(shape),
    //         MeshMaterial2d(materials.add(color)),
    //         Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
    //             -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //             0.0,
    //             0.0,
    //         ),
    //     ));
    // }

    //for (i, shape) in shapes.into_iter().enumerate() {
    let entities = shapes
        .into_iter()
        .enumerate()
        .map(|(i, shape)| {
            // Distribute colors evenly across the rainbow.
            let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

            commands
                .spawn((
                    RigidBody::Dynamic,
                    Collider::circle(50.),
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(
                        -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                        0.0,
                        0.0,
                    ),
                    //LinearVelocity(Vec2::new(1000., 1000.)),
                    ExternalForce::new(Vec2::new(100000., 100000.)),
                    // ExternalForce::default().apply_force_at_point(
                    //     Vec2::new(100., 100.),
                    //     Vec2::ZERO,
                    //     Vec2::ZERO,
                    // ),
                ))
                .observe(drag_n_drop)
                .observe(drag_start)
                .observe(drag_end)
                .id()
        })
        .collect::<Vec<_>>();

    commands.spawn(
        DistanceJoint::new(*entities.get(0).unwrap(), *entities.get(1).unwrap())
            .with_limits(200., 300.)
            .with_linear_velocity_damping(0.5),
    );

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

fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut Mesh2d,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut Transform,
    )>,
) {
    // let t = (ops::sin(time.elapsed_secs()) + 1.) / 2.;

    // for (mut mesh, mut material, mut transform) in &mut query {
    //     transform.translation.y = 600. * (t - 0.5);
    // }
}

fn drag_n_drop(
    trigger: Trigger<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let mut transform = transforms.get_mut(trigger.entity()).unwrap();

    let (camera, camera_transform) = camera_q.get_single().expect("Expected a single camera");
    let world_pos = camera
        .viewport_to_world_2d(camera_transform, trigger.event().pointer_location.position)
        .expect("Camera's projection matrix is invalid");
    transform.translation = Vec3::new(world_pos.x, world_pos.y, transform.translation.z);
}

fn drag_start(trigger: Trigger<Pointer<DragStart>>, mut commands: Commands) {
    // commands
    //     .entity(trigger.event().target)
    //     .insert(RigidBodyDisabled);
}

fn drag_end(trigger: Trigger<Pointer<DragEnd>>, mut commands: Commands) {
    // commands
    //     .entity(trigger.event().target)
    //     .remove::<RigidBodyDisabled>();
}
