use nalgebra::{Quaternion, Vector3};
use num_traits::real::Real;
use vek::{CubicBezier3, Vec3};

/// A `SpaceTrace` defines a path through 3D space.
#[derive(Debug)]
pub struct SpaceTrace<T: Real> {
    curve: CubicBezier3<T>,
    start_quat: Quaternion<T>,
    end_quat: Quaternion<T>,
}

impl<T: Real + Default> SpaceTrace<T> {
    /// Create a new `SpaceTrace` using a cubic bezier curve, and two quaternions defining start and end rotations.
    pub fn new(
        start: Vector3<T>,
        start_rot: Quaternion<T>,
        control_1: Vector3<T>,
        control_2: Vector3<T>,
        end: Vector3<T>,
        end_rot: Quaternion<T>,
    ) -> Self {
        Self {
            curve: CubicBezier3 {
                start: Vec3::from_slice(start.as_slice()),
                ctrl0: Vec3::from_slice(control_1.as_slice()),
                ctrl1: Vec3::from_slice(control_2.as_slice()),
                end: Vec3::from_slice(end.as_slice()),
            },
            start_quat: start_rot,
            end_quat: end_rot,
        }
    }
}
