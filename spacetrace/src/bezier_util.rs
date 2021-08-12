//! Utilities for working with bezier curves

use num_traits::real::Real;
use vek::{CubicBezier3, QuadraticBezier3, Vec3};

/// Common trait for all bezier curve types
pub trait BezierCurve<T: Real> {
    /// Get the curve value at progress `k`
    fn eval(&self, k: T) -> Vec3<T>;
}

impl<T: Real> BezierCurve<T> for CubicBezier3<T> {
    fn eval(&self, k: T) -> Vec3<T> {
        self.evaluate(k)
    }
}

impl<T: Real> BezierCurve<T> for QuadraticBezier3<T> {
    fn eval(&self, k: T) -> Vec3<T> {
        self.evaluate(k)
    }
}

impl<T: Real> BezierCurve<T> for &dyn BezierCurve<T> {
    fn eval(&self, k: T) -> Vec3<T> {
        (*self).eval(k)
    }
}