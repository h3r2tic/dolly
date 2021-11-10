use glam::Quat;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

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

impl RigDriver for Rotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            position: params.parent.position,
            rotation: self.rotation,
        }
    }
}
