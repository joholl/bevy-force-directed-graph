use super::utils::{ClampF32Range, FiniteOr as _};
use crate::force_directed_graph::common::{MouseLocked, NodePhysics};
use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::system::{Query, Res, ResMut, Resource},
    time::Time,
    transform::components::Transform,
};
use core::{f32, panic};
use std::collections::VecDeque;

#[derive(Resource, Debug, Default)]
pub struct VerletRes {
    time_deltas: VecDeque<f32>,
}

impl VerletRes {
    const NUM_TIME_STEPS: usize = 2;

    /// Add a new time delta to the list of time deltas.
    /// If the list is empty, fill it completely with the same delta value.
    pub fn update(&mut self, delta: f32) {
        if self.time_deltas.is_empty() {
            for _ in 0..Self::NUM_TIME_STEPS {
                self.time_deltas.push_back(delta);
            }
        } else {
            self.time_deltas.push_front(delta);
            self.time_deltas.truncate(Self::NUM_TIME_STEPS);
        }
    }

    pub fn average_time_delta(&self, n: usize) -> f32 {
        if self.time_deltas.is_empty() {
            panic!("Cannot access VerletRes before first update");
        }

        let sum = self
            .time_deltas
            .iter()
            .take(n)
            .sum::<f32>()
            .clamp_f32_range();
        let count = self.time_deltas.len().min(n) as f32;

        (sum / count).clamp_f32_range()
    }

    /// We need to account for non-constant time steps
    /// https://en.wikipedia.org/wiki/Verlet_integration#Non-constant_time_differences
    pub fn velocity_factor(&self) -> f32 {
        if self.time_deltas.is_empty() {
            panic!("Cannot access VerletRes before first update");
        }

        (self.time_deltas[0] / self.time_deltas[1]).finite_or(1.0)
    }

    pub fn delta_secs(&self) -> f32 {
        if self.time_deltas.is_empty() {
            panic!("Cannot access VerletRes before first update");
        }

        self.time_deltas[0]
    }

    /// We need to account for non-constant time steps
    /// https://en.wikipedia.org/wiki/Verlet_integration#Non-constant_time_differences
    pub fn delta_secs_squared(&self) -> f32 {
        if self.time_deltas.is_empty() {
            panic!("Cannot access VerletRes before first update");
        }

        ((0.5 * (self.time_deltas[0] + self.time_deltas[1]).clamp_f32_range()).clamp_f32_range()
            * self.time_deltas[0])
            .clamp_f32_range()
    }
}

pub struct VerletPlugin {
    pub velocity_decay: f32,
}

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        // TODO add bevy::Time if not added already?
        app.insert_resource(VerletRes::default())
            .add_systems(PreUpdate, (apply_velocity(self.velocity_decay),));
    }
}

/// Given a node position (Transform) and its previous position (NodePhysics),
/// this function calculates the new position using Verlet integration.
///
/// Usually, the Verlet integration considers the acceleration (based on some
/// forces and a mass of 1). However, here the forces change the current
/// position directly, so we do not need to consider (i.e. save and calculate)
/// the acceleration.
///
/// Inputs:
///  - node position: result of last Verlet integration step
///  - previous position: result of the Verlet integration step before that
///
/// Outputs:
///  - node position: new position of the node
///  - previous position: result of the last Verlet integration step
pub fn apply_velocity(
    velocity_decay: f32,
) -> impl FnMut(
    Query<'_, '_, (&mut Transform, &mut NodePhysics, Option<&mut MouseLocked>)>,
    Res<'_, Time>,
    ResMut<'_, VerletRes>,
) {
    move |mut nodes_q: Query<(&mut Transform, &mut NodePhysics, Option<&mut MouseLocked>)>,
          time: Res<Time>,
          mut verlet: ResMut<VerletRes>| {
        // First, update time steps
        verlet.update(if time.delta_secs() != 0.0 {
            // TODO
            time.delta_secs()
        } else {
            0.01
        });

        // Do verlet integration
        nodes_q
            .iter_mut()
            .for_each(|(mut transform, mut node_physics, mouse_locked)| {
                // Current state
                let position = transform.translation.truncate();
                let previous_position = node_physics.previous_position;

                // Verlet integration step:
                // we do not store the velocity, but we can approximate it from the previous and current position
                let position_next = position
                    + (position - previous_position) * verlet.velocity_factor() * velocity_decay;
                let previous_position_next = position;

                // Update state
                if let Some(mut mouse_locked) = mouse_locked {
                    // Mouse locked, only update mouse velocity
                    // Use a moving average since mouse events do not appear not reliably
                    mouse_locked.velocity =
                        0.90 * mouse_locked.velocity + 0.10 * (position_next - position);
                } else {
                    // Normal case
                    transform.translation = position_next.extend(transform.translation.z);
                    #[cfg(debug_assertions)]
                    assert!(transform.is_finite(), "Not finite: {:?}", transform);
                }
                node_physics.previous_position = previous_position_next;
            });
    }
}
