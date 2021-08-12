//! A 3D path solving library

#![no_std]
#![warn(missing_docs)]

extern crate core as std;

#[macro_use]
pub extern crate serde;

pub mod trace;
pub use crate::trace::SpaceTrace;
pub mod poserot;
pub use crate::poserot::PoseRot;
pub(crate) mod bezier_util;