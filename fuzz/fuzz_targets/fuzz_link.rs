#![no_main]

use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::time;
use bevy::transform::components::Transform;
use bevy_force_directed_graph::force_directed_graph::common::{NodeLink, NodePhysics};
use bevy_force_directed_graph::force_directed_graph::forces::link::apply_link_force;
use bevy_force_directed_graph::force_directed_graph::utils::FiniteOr as _;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut slice = data;

    let simulation_steps = *slice.first().unwrap_or(&1);
    slice = &slice[1.min(slice.len())..];

    // Setup app
    let mut app = App::new();
    app.add_plugins(time::TimePlugin);
    app.add_systems(Update, apply_link_force);

    // Decide node count
    let node_bytes_len = std::mem::size_of::<[f32; 3]>();
    let (nodes_links_factor_slice, slice) = slice
        .split_at_checked(std::mem::size_of::<u16>())
        .unwrap_or((slice, &[]));
    let nodes_links_factor =
        u16::from_ne_bytes(nodes_links_factor_slice.try_into().unwrap_or([0, 0])) as f32
            / u16::MAX as f32;
    let nodes_bytes_len = (slice.len() as f32 * nodes_links_factor) as usize;

    // Add nodes
    let node_entities = slice[..nodes_bytes_len]
        .chunks_exact(node_bytes_len)
        .map(|slice| {
            let transform = Transform::from_translation(Vec3::new(
                f32::from_ne_bytes(slice[0..4].try_into().unwrap()).finite_or(0.0),
                f32::from_ne_bytes(slice[4..8].try_into().unwrap()).finite_or(0.0),
                f32::from_ne_bytes(slice[8..12].try_into().unwrap()).finite_or(0.0),
            ));
            app.world_mut()
                .spawn((NodePhysics::from_transform(transform), transform))
                .id()
        })
        .collect::<Vec<_>>();
    let slice = &slice[nodes_bytes_len..];

    if node_entities.is_empty() {
        return;
    }

    // Add links
    slice
        .chunks_exact(2 * std::mem::size_of::<u16>() + std::mem::size_of::<f32>())
        .for_each(|slice| {
            let source_idx =
                u16::from_ne_bytes(slice[0..2].try_into().unwrap()) as usize % node_entities.len();
            let target_idx =
                u16::from_ne_bytes(slice[2..4].try_into().unwrap()) as usize % node_entities.len();
            if source_idx == target_idx {
                return;
            }
            app.world_mut().spawn((NodeLink {
                source: node_entities[source_idx],
                target: node_entities[target_idx],
                target_distance: f32::from_ne_bytes(slice[4..8].try_into().unwrap()).finite_or(1.0),
            },));
        });

    // Run systems
    for _ in 0..simulation_steps {
        app.update();
    }
});
