//! A 3D path solving library

#![no_std]
#![warn(missing_docs)]

extern crate core as std;

#[macro_use]
pub extern crate serde;

mod trace;
pub use crate::trace::SpaceTrace;
mod poserot;
pub use crate::poserot::PoseRot;