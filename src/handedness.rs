use std::fmt::Debug;

use glam::Vec3;

pub trait Handedness: Clone + Copy + Debug + 'static {
    const FORWARD_Z_SIGN: f32;

    fn forward() -> Vec3 {
        Vec3::Z * Self::FORWARD_Z_SIGN
    }

    fn right_handed() -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded {}

impl Handedness for LeftHanded {
    const FORWARD_Z_SIGN: f32 = 1.0;

    fn right_handed() -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded {}

impl Handedness for RightHanded {
    const FORWARD_Z_SIGN: f32 = -1.0;

    fn right_handed() -> bool {
        true
    }
}
