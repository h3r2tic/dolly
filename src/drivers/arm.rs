use std::marker::PhantomData;

use glam::{Quat, Vec3};

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    pub offset: mint::Vector3<f32>,
}

impl Arm {
    pub fn new<V>(offset: V) -> Self
    where
        V: Into<mint::Vector3<f32>>,
    {
        let offset = offset.into();

        Self { offset }
    }
}

impl<H: Handedness> RigDriver<H> for Arm {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        let parent_position: Vec3 = params.parent.position.into();
        let parent_rotation: Quat = params.parent.rotation.into();
        let offset: Vec3 = self.offset.into();

        let position = parent_position + parent_rotation * offset;

        Transform {
            rotation: params.parent.rotation,
            position: position.into(),
            phantom: PhantomData,
        }
    }
}
