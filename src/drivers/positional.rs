use glam::{Quat, Vec3};

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

#[derive(Default)]
pub struct Positional {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Positional {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
    }

    pub fn set_position_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }
}

impl RigDriver for Positional {
    fn update(&mut self, _: RigUpdateParams) -> Transform {
        Transform {
            translation: self.position,
            rotation: self.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
