use core::fmt::Debug;
use glam::{Quat, Vec3};

/// A thin wrapper over a `Vec3` and a `Quat`
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Transform {
    ///
    pub fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }

    ///
    pub fn into_position_rotation(self) -> (Vec3, Quat) {
        (self.position, self.rotation)
    }

    /// +X
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// +Y
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// -Z
    pub fn forward(&self, right_handed: bool) -> Vec3 {
        let sign = if right_handed { -1.0 } else { 1.0 };
        self.rotation * Vec3::Z * sign
    }

    ///
    pub const IDENTITY: Transform = Transform {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
}
