use bevy::math::{Vec2, Vec3};

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

pub trait ClampFiniteOr {
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
