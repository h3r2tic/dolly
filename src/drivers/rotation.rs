use std::marker::PhantomData;

use glam::Quat;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Directly sets the rotation of the camera
#[derive(Default, Debug)]
pub struct Rotation {
    pub rotation: Quat,
}

impl Rotation {
    pub fn new(rotation: Quat) -> Self {
        Self { rotation }
    }
}

impl<H: Handedness> RigDriver<H> for Rotation {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: params.parent.position,
            rotation: self.rotation,
            ty: PhantomData,
        }
    }
}
