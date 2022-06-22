use std::fmt::Debug;

use glam::Vec3;

pub trait Handedness: Clone + Copy + Debug + 'static {
    const FORWARD_Z_SIGN: f32;
    // `glam::const_vec3!` is deprecated  since 0.21 and to be replaced with
    // `glam::Vec3::new(0.0, 0.0, Self::FORWARD_Z_SIGN)`. Consider replacing
    // it when bumping glam's minimum version to 0.21.
    #[allow(deprecated)]
    const FORWARD: Vec3 = glam::const_vec3!([0.0, 0.0, Self::FORWARD_Z_SIGN]);

    fn right_from_up_and_forward(up: Vec3, forward: Vec3) -> Vec3;
    fn up_from_right_and_forward(right: Vec3, forward: Vec3) -> Vec3;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded;

impl Handedness for LeftHanded {
    const FORWARD_Z_SIGN: f32 = 1.0;

    fn right_from_up_and_forward(up: Vec3, forward: Vec3) -> Vec3 {
        up.cross(forward)
    }

    fn up_from_right_and_forward(right: Vec3, forward: Vec3) -> Vec3 {
        forward.cross(right)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded;

impl Handedness for RightHanded {
    const FORWARD_Z_SIGN: f32 = -1.0;

    fn right_from_up_and_forward(up: Vec3, forward: Vec3) -> Vec3 {
        forward.cross(up)
    }

    fn up_from_right_and_forward(right: Vec3, forward: Vec3) -> Vec3 {
        right.cross(forward)
    }
}
