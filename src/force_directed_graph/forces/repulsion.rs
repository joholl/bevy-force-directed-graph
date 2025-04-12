use crate::force_directed_graph::{
    common::{alpha, MouseLocked, NodePhysics},
    utils::{FiniteOr as _, FiniteOrRandom as _},
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
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
        let direction = direction.normalize().finite_or_random_normalized();

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
            #[cfg(debug_assertions)]
            assert!(a_transform.is_finite(), "Not finite: {:?}", a_transform);
        }
        if b_mouse_locked.is_none() {
            b_transform.translation += force.extend(0.0);
            #[cfg(debug_assertions)]
            assert!(b_transform.is_finite(), "Not finite: {:?}", b_transform);
        }
    }
}
