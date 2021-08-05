use glam::{Quat, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl Transform {
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn into_translation_rotation(self) -> (Vec3, Quat) {
        (self.translation, self.rotation)
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    pub const IDENTITY: Transform = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
}
