use bevy::math::{Vec2, Vec3};
use rand::rngs::SmallRng;
use rand::{Rng as _, SeedableRng as _};

pub trait MapNonFinite {
    fn map_nonfinite(self, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized;
}
impl MapNonFinite for f32 {
    fn map_nonfinite(self, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if self.is_finite() {
            self
        } else {
            f(self)
        }
    }
}
impl MapNonFinite for f64 {
    fn map_nonfinite(self, f: impl FnOnce(Self) -> Self) -> Self {
        if self.is_finite() {
            self
        } else {
            f(self)
        }
    }
}
impl MapNonFinite for Vec2 {
    fn map_nonfinite(self, f: impl FnOnce(Self) -> Self) -> Self {
        if self.is_finite() {
            self
        } else {
            f(self)
        }
    }
}
impl MapNonFinite for Vec3 {
    fn map_nonfinite(self, f: impl FnOnce(Self) -> Self) -> Self {
        if self.is_finite() {
            self
        } else {
            f(self)
        }
    }
}

pub trait FiniteOr {
    fn finite_or(self, v: Self) -> Self;
}
impl<T: MapNonFinite> FiniteOr for T {
    fn finite_or(self, v: Self) -> Self {
        self.map_nonfinite(|_| v)
    }
}
pub trait FiniteOrRandom {
    fn finite_or_random_normalized(self) -> Self;
}
impl FiniteOrRandom for Vec2 {
    fn finite_or_random_normalized(self) -> Self {
        self.map_nonfinite(|_| {
            let mut rng = SmallRng::seed_from_u64(0);
            let angle = rng.random_range(0.0..std::f32::consts::TAU); // TAU = 2π
            Vec2::new(angle.cos(), angle.sin())
        })
    }
}

pub trait ClampFiniteOr {
    #[allow(dead_code)]
    fn clamp_finite_or(self, min: Self, max: Self, v: Self) -> Self;
}
impl ClampFiniteOr for f32 {
    fn clamp_finite_or(self, min: Self, max: Self, v: Self) -> Self {
        self.clamp(min, max).finite_or(v)
    }
}
impl ClampFiniteOr for f64 {
    fn clamp_finite_or(self, min: Self, max: Self, v: Self) -> Self {
        self.clamp(min, max).finite_or(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_tests_for_map_noninfinite_and_finilize_or {
        ($($name:ident($input_object:expr, $input_default:expr) => $expected:expr),* $(,)?) => {
            $(
                paste::item! {
                    #[test]
                    fn [< test_map_nonfinite_ $name >] () {
                        assert_eq!($input_object.map_nonfinite(|| $input_default), $expected);
                    }
                }

                paste::item! {
                    #[test]
                    fn [< test_finite_or_ $name >] () {
                        assert_eq!($input_object.finite_or($input_default), $expected);
                    }
                }
            )*
        };
    }

    generate_tests_for_map_noninfinite_and_finilize_or! {
        f32_finite_one(1.0_f32, 10.0) => 1.0,
        f32_finite_min(f32::MIN, 10.0) => f32::MIN,
        f32_finite_max(f32::MAX, 10.0) => f32::MAX,
        f32_infinite_nan(f32::NAN, 10.0) => 10.0,
        f32_infinite_inf(f32::INFINITY, 10.0) => 10.0,
        f32_infinite_neginf(f32::NEG_INFINITY, 10.0) => 10.0,
        f64_finite_one(1.0_f64, 10.0) => 1.0,
        f64_finite_min(f64::MIN, 10.0) => f64::MIN,
        f64_finite_max(f64::MAX, 10.0) => f64::MAX,
        f64_infinite_nan(f64::NAN, 10.0) => 10.0,
        f64_infinite_inf(f64::INFINITY, 10.0) => 10.0,
        f64_infinite_neginf(f64::NEG_INFINITY, 10.0) => 10.0,
        vec2_finite(Vec2::new(1.0, 2.0), Vec2::new(10.0, 20.0)) => Vec2::new(1.0, 2.0),
        vec2_infinite_one(Vec2::new(f32::NAN, 2.0), Vec2::new(10.0, 20.0)) => Vec2::new(10.0, 20.0),
        vec2_infinite_one_nan(Vec2::new(1.0, f32::NAN), Vec2::new(10.0, 20.0)) => Vec2::new(10.0, 20.0),
        vec2_infinite_inf_two(Vec2::new(f32::INFINITY, 2.0), Vec2::new(10.0, 20.0)) => Vec2::new(10.0, 20.0),
        vec2_infinite_fin_nan(Vec2::new(f32::INFINITY, f32::NAN), Vec2::new(10.0, 20.0)) => Vec2::new(10.0, 20.0),
        vec2_infinite_neginf_neginf(Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY), Vec2::new(10.0, 20.0)) => Vec2::new(10.0, 20.0),
        vec3_finite(Vec3::new(1.0, 2.0, 3.0), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(1.0, 2.0, 3.0),
        vec3_infinite_nan_two_three(Vec3::new(f32::NAN, 2.0, 3.0), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
        vec3_infinite_one_nan_three(Vec3::new(1.0, f32::NAN, 3.0), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
        vec3_infinite_one_two_nan(Vec3::new(1.0, 2.0, f32::NAN), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
        vec3_infinite_inf_two_three(Vec3::new(f32::INFINITY, 2.0, 3.0), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
        vec3_infinite_fin_nan_three(Vec3::new(f32::INFINITY, f32::NAN, 3.0), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
        vec3_infinite_neginf_neginf_neginf(Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY), Vec3::new(10.0, 20.0, 30.0)) => Vec3::new(10.0, 20.0, 30.0),
    }

    #[test]
    fn test_finite_or_random_normalized_finite() {
        assert_eq!(
            Vec2::new(1.0, 2.0).finite_or_random_normalized(),
            Vec2::new(1.0, 2.0)
        );
    }

    #[test]
    fn test_finite_or_random_normalized_infinite() {
        assert_ne!(
            Vec2::new(1.0, f32::NAN).finite_or_random_normalized(),
            Vec2::new(1.0, 2.0)
        );
    }

    #[test]
    fn test_clamp_finite_or_finite_self() {
        assert_eq!(0.5f32.clamp_finite_or(0.0, 1.0, 10.0), 0.5);
    }

    #[test]
    fn test_clamp_finite_or_finite_min() {
        assert_eq!((-1.0f32).clamp_finite_or(0.0, 1.0, 10.0), 0.0);
    }

    #[test]
    fn test_clamp_finite_or_finite_max() {
        assert_eq!(2.0f32.clamp_finite_or(0.0, 1.0, 10.0), 1.0);
    }

    #[test]
    fn test_clamp_finite_or_infinite() {
        assert_eq!(f32::NAN.clamp_finite_or(0.0, 1.0, 10.0), 10.0);
    }
}
