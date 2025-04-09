use bevy::math::{Vec2, Vec3};
use rand::Rng as _;

pub trait MapNonFinite {
    fn map_nonfinite(self, f: impl FnOnce() -> Self) -> Self;
}
impl MapNonFinite for f32 {
    fn map_nonfinite(self, f: impl FnOnce() -> Self) -> Self {
        if self.is_finite() {
            self
        } else {
            f()
        }
    }
}
impl MapNonFinite for f64 {
    fn map_nonfinite(self, f: impl FnOnce() -> Self) -> Self {
        if self.is_finite() {
            self
        } else {
            f()
        }
    }
}
impl MapNonFinite for Vec2 {
    fn map_nonfinite(self, f: impl FnOnce() -> Self) -> Self {
        if self.x.is_finite() && self.y.is_finite() {
            self
        } else {
            f()
        }
    }
}
impl MapNonFinite for Vec3 {
    fn map_nonfinite(self, f: impl FnOnce() -> Self) -> Self {
        if self.x.is_finite() && self.y.is_finite() && self.z.is_finite() {
            self
        } else {
            f()
        }
    }
}

pub trait FiniteOr {
    fn finite_or(self, v: Self) -> Self;
}
impl<T: MapNonFinite> FiniteOr for T {
    fn finite_or(self, v: Self) -> Self {
        self.map_nonfinite(|| v)
    }
}
pub trait FiniteOrRandom {
    fn finite_or_random_normalized(self) -> Self;
}
impl FiniteOrRandom for Vec2 {
    fn finite_or_random_normalized(self) -> Self {
        self.map_nonfinite(|| {
            let mut rng = rand::rng();
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
