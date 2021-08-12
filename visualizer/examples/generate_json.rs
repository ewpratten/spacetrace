//! This example is used to generate test JSON data for the visualizer.

use spacetrace::{PoseRot, SpaceTrace};
use vek::{Quaternion, Vec3};

fn main() {
    // Generate a test trace
    let start = PoseRot::new(Vec3::zero(), Quaternion::identity());
    let end = PoseRot::new(
        Vec3::new(4.0, 4.0, 4.0),
        Quaternion::from_xyzw(0.545, -0.039, 0.572, 0.612),
    );
    let st = SpaceTrace::new_cubic_bezier(
        start,
        Vec3::new(2.0, 6.0, 3.0),
        Vec3::new(4.0, 6.0, 3.0),
        end,
    );

    // Print the data
    println!("{}", serde_json::to_string(&st).unwrap());
}
