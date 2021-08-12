//! Utilities for expressing pairs of Vectors and Quaternions.

use std::ops::Add;

use core::fmt::Debug;
use num_traits::{Float, One, Zero};
use vek::{Quaternion, Vec3};

/// A PoseRot is a collection of a pose and a quaternion.
/// Used to describe an object in space, looking in a direction.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PoseRot<T>
where
    T: Clone + PartialEq,
{
    /// Pose in space
    pub pose: Vec3<T>,
    /// Rotation
    pub rotation: Quaternion<T>,
}

impl<T: Clone + PartialEq + Float> PoseRot<T> {
    /// Create a new PoseRot from its components.
    pub fn new(pose: Vec3<T>, rotation: Quaternion<T>) -> Self {
        Self {
            pose,
            rotation: rotation.normalized(),
        }
    }
}

impl<T: Zero + One + Clone + PartialEq + Float> Default for PoseRot<T> {
    /// Create a zeroed PoseRot
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Zero + One + Clone + PartialEq + Float> Add for PoseRot<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pose: self.pose + rhs.pose,
            rotation: (self.rotation + rhs.rotation).normalized(),
        }
    }
}

impl<T: Zero + One + Clone + PartialEq + Float> Zero for PoseRot<T> {
    fn zero() -> Self {
        Self {
            pose: Vec3::zero(),
            rotation: Quaternion::identity(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let pr: PoseRot<f32> = PoseRot::zero();
        assert_eq!(pr.pose, Vec3::zero());
        assert_eq!(pr.rotation, Quaternion::identity());
    }

    #[test]
    fn test_default() {
        let pr: PoseRot<f32> = PoseRot::default();
        assert_eq!(pr.pose, Vec3::zero());
        assert_eq!(pr.rotation, Quaternion::identity());
    }

    #[test]
    fn test_add() {
        let pr1: PoseRot<f32> = PoseRot::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quaternion::from_xyzw(1.0, 2.0, 3.0, 4.0),
        );
        let pr2: PoseRot<f32> = PoseRot::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quaternion::from_xyzw(1.0, 2.0, 3.0, 4.0),
        );
        let result = pr1 + pr2;
        assert_eq!(result.pose, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(result.rotation, Quaternion::from_xyzw(0.1825742, 0.3651484, 0.5477226, 0.7302968));
    }
}
