//! Main definitions for the SpaceTrace

use num_traits::{real::Real, Float};
use vek::{Clamp, CubicBezier3, Lerp, QuadraticBezier3, Quaternion, Vec3};

use crate::{bezier_util::*, PoseRot};

/// A `SpaceTrace` defines a path through 3D space.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceTrace<T: Real, C: BezierCurve<T>> {
    curve: C,
    start_quat: Quaternion<T>,
    end_quat: Quaternion<T>,
}

impl<T: Real + Default> SpaceTrace<T, CubicBezier3<T>> {
    /// Create a new `SpaceTrace` using a cubic bezier curve,
    /// and two quaternions defining start and end rotations.
    pub fn new_cubic_bezier(
        start: PoseRot<T>,
        control_1: Vec3<T>,
        control_2: Vec3<T>,
        end: PoseRot<T>,
    ) -> Self {
        Self {
            curve: CubicBezier3 {
                start: start.pose,
                ctrl0: control_1,
                ctrl1: control_2,
                end: end.pose,
            },
            start_quat: start.rotation,
            end_quat: end.rotation,
        }
    }
}

impl<T: Real + Default> SpaceTrace<T, QuadraticBezier3<T>> {
    /// Create a new `SpaceTrace` using a quadratic bezier curve,
    /// and two quaternions defining start and end rotations.
    pub fn new_quadratic_bezier(start: PoseRot<T>, control: Vec3<T>, end: PoseRot<T>) -> Self {
        Self {
            curve: QuadraticBezier3 {
                start: start.pose,
                ctrl: control,
                end: end.pose,
            },
            start_quat: start.rotation,
            end_quat: end.rotation,
        }
    }
}

impl<T: Real + Default + Lerp<T, Output = T> + Clamp + Float, C: BezierCurve<T>> SpaceTrace<T, C> {
    /// Evaluate the `SpaceTrace` at a given progress (from `0` to `1`).
    ///
    /// This will return both a pose along the internal curve, and a quaternion
    /// rotation based on the *Spherical Linear Interpolation* between the start and end rotations.
    pub fn evaluate(&self, progress: T) -> PoseRot<T> {
        // Get the pose progress
        let pose = self.curve.eval(progress);

        // Get the quaternion progress
        let quat = Quaternion::slerp(self.start_quat, self.end_quat, progress);

        // Build poserot
        PoseRot::new(pose, quat)
    }
}

#[cfg(test)]
mod tests {

    use vek::approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_quadratic_start_end() {
        let start = PoseRot::new(Vec3::zero(), Quaternion::identity());
        let end = PoseRot::new(
            Vec3::new(4.0, 4.0, 4.0),
            Quaternion::from_xyzw(0.545, -0.039, 0.572, 0.612),
        );
        let st = SpaceTrace::new_quadratic_bezier(start, Vec3::new(2.0, 6.0, 3.0), end);
        assert_eq!(st.evaluate(0.0).pose, Vec3::zero());
        assert_eq!(st.evaluate(1.0).pose, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(st.evaluate(0.0).rotation, Quaternion::identity());
        assert_relative_eq!(
            st.evaluate(1.0).rotation,
            Quaternion::from_xyzw(0.545, -0.039, 0.572, 0.612),
            epsilon = 0.001
        );
    }

    #[test]
    fn test_cubic_start_end() {
        let start = PoseRot::new(Vec3::zero(), Quaternion::identity());
        let end = PoseRot::new(
            Vec3::new(4.0, 4.0, 4.0),
            Quaternion::from_xyzw(0.545, -0.039, 0.572, 0.612),
        );
        let st = SpaceTrace::new_cubic_bezier(start, Vec3::new(2.0, 6.0, 3.0), Vec3::new(4.0, 6.0, 3.0), end);
        assert_eq!(st.evaluate(0.0).pose, Vec3::zero());
        assert_eq!(st.evaluate(1.0).pose, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(st.evaluate(0.0).rotation, Quaternion::identity());
        assert_relative_eq!(
            st.evaluate(1.0).rotation,
            Quaternion::from_xyzw(0.545, -0.039, 0.572, 0.612),
            epsilon = 0.001
        );
    }
}
