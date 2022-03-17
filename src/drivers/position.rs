use std::marker::PhantomData;

use glam::Vec3;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Directly sets the position of the camera
#[derive(Default, Debug)]
pub struct Position {
    pub position: Vec3,
}

impl Position {
    ///
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    /// Add the specified vector to the position of this component
    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
    }
}

impl<H: Handedness + 'static> RigDriver<H> for Position {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: self.position,
            rotation: params.parent.rotation,
            ty: PhantomData,
        }
    }
}
