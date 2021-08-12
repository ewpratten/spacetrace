//! Utilities for expressing pairs of Vectors and Quaternions.

use std::ops::Add;

use core::fmt::Debug;
use num_traits::{One, Zero};
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

impl<T: Clone + PartialEq> PoseRot<T> {
    /// Create a new PoseRot from its components.
    pub fn new(pose: Vec3<T>, rotation: Quaternion<T>) -> Self {
        Self { pose, rotation }
    }
}

impl<T: Zero + One + Clone + PartialEq> Default for PoseRot<T> {
    /// Create a zeroed PoseRot
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Zero + One + Clone + PartialEq> Add for PoseRot<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pose: self.pose + rhs.pose,
            rotation: self.rotation + rhs.rotation,
        }
    }
}

impl<T: Zero + One + Clone + PartialEq> Zero for PoseRot<T> {
    fn zero() -> Self {
        Self {
            pose: Vec3::zero(),
            rotation: Quaternion::from_xyzw(T::zero(), T::zero(), T::zero(), T::one()),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}
