use std::fmt::Debug;

pub trait Handedness: Clone + Copy + Debug {
    fn z_sign() -> f32;
    fn right_handed() -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded {}

impl Handedness for LeftHanded {
    fn z_sign() -> f32 {
        1.0
    }

    fn right_handed() -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded {}

impl Handedness for RightHanded {
    fn z_sign() -> f32 {
        -1.0
    }

    fn right_handed() -> bool {
        true
    }
}
