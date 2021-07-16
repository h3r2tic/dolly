use glam::Vec3;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

pub struct Arm {
    pub offset: Vec3,
}

impl Arm {
    pub fn new(offset: Vec3) -> Self {
        Self { offset }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            rotation: params.parent.rotation,
            translation: params.parent.translation + params.parent.rotation * self.offset,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
