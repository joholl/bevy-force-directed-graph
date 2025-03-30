
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};
use rand::Rng;

use crate::{
    common::{alpha, MouseLocked, NodePhysics},
    utils::{FiniteOr as _, MapNonFinite},
};

pub fn apply_repulsion_force(
    mut transforms_q: Query<(&mut Transform, Option<&MouseLocked>), With<NodePhysics>>,
    time: Res<Time>,
) {
    let mut combinations = transforms_q.iter_combinations_mut::<2>();
    while let Some([(mut a_transform, a_mouse_locked), (mut b_transform, b_mouse_locked)]) =
        combinations.fetch_next()
    {
        let direction = b_transform.translation.truncate() - a_transform.translation.truncate();
        let distance = direction.length();

        // deal with NaN and zero
        // this also prevents overly big forces if nodes get too close
        let distance = distance.finite_or(10.0).clamp(10.0, f32::MAX);

        // if the direction vector is zero, normalizing will lead to NaN (-> take a random direction)
        let direction = direction.normalize().map_nonfinite(|| {
            let mut rng = rand::rng();
            let angle = rng.random_range(0.0..std::f32::consts::TAU); // TAU = 2π
            Vec2::new(angle.cos(), angle.sin())
        });

        // Calculate the repulsion based on the distance
        let strength = 100000.0;
        let force = alpha(time.delta_secs()) * strength / (distance * distance) * direction;

        // deal with NaN and too big forces
        //let force = force;
        #[cfg(debug_assertions)]
        if force.x.is_nan() || force.y.is_nan() {
            dbg!(a_transform);
            dbg!(b_transform);
            dbg!(direction);
            dbg!(distance);
            dbg!(distance * distance);
            dbg!(force);

            panic!("NaN in b_transform");
        }

        // Update the positions of both nodes
        if a_mouse_locked.is_none() {
            a_transform.translation -= force.extend(0.0);
        }
        if b_mouse_locked.is_none() {
            b_transform.translation += force.extend(0.0);
        }

        #[cfg(debug_assertions)]
        if a_transform.translation.x.is_nan()
            || a_transform.translation.y.is_nan()
            || a_transform.translation.z.is_nan()
            || b_transform.translation.x.is_nan()
            || b_transform.translation.y.is_nan()
            || b_transform.translation.z.is_nan()
        {
            dbg!(a_transform);
            dbg!(b_transform);
            panic!("NaN in transform");
        }
    }
}
