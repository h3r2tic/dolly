use std::marker::PhantomData;

use glam::Quat;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Directly sets the rotation of the camera
#[derive(Debug)]
pub struct Rotation {
    pub rotation: mint::Quaternion<f32>,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            rotation: Quat::default().into(),
        }
    }
}

impl Rotation {
    pub fn new<Q>(rotation: Q) -> Self
    where
        Q: Into<mint::Quaternion<f32>>,
    {
        let rotation = rotation.into();

        Self { rotation }
    }
}

impl<H: Handedness> RigDriver<H> for Rotation {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: params.parent.position,
            rotation: self.rotation,
            phantom: PhantomData,
        }
    }
}
