#![no_main]

use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::time;
use bevy::transform::components::Transform;
use bevy_force_directed_graph::force_directed_graph::common::NodePhysics;
use bevy_force_directed_graph::force_directed_graph::forces::repulsion::apply_repulsion_force;
use bevy_force_directed_graph::force_directed_graph::utils::FiniteOr as _;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut slice = data;

    let simulation_steps = *slice.first().unwrap_or(&1);
    slice = &slice[1.min(slice.len())..];

    // Setup app
    let mut app = App::new();
    app.add_plugins(time::TimePlugin);
    app.add_systems(Update, apply_repulsion_force);

    // Add nodes
    slice
        .chunks_exact(std::mem::size_of::<[f32; 3]>())
        .for_each(|slice| {
            let transform = Transform::from_translation(Vec3::new(
                f32::from_ne_bytes(slice[0..4].try_into().unwrap()).finite_or(0.0),
                f32::from_ne_bytes(slice[4..8].try_into().unwrap()).finite_or(0.0),
                f32::from_ne_bytes(slice[8..12].try_into().unwrap()).finite_or(0.0),
            ));
            app.world_mut()
                .spawn((NodePhysics::from_transform(transform), transform));
        });

    // Run systems
    for _ in 0..simulation_steps {
        app.update();
    }
});
