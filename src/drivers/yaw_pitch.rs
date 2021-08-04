use glam::{EulerRot, Quat};

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

pub struct YawPitch {
    pub yaw_degrees: f32,
    pub pitch_degrees: f32,
}

impl Default for YawPitch {
    fn default() -> Self {
        Self::new()
    }
}

impl YawPitch {
    pub fn new() -> Self {
        Self {
            yaw_degrees: 0.0,
            pitch_degrees: 0.0,
        }
    }

    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.set_rotation(rotation);
        self
    }

    pub fn yaw_degrees(mut self, yaw_degrees: f32) -> Self {
        self.yaw_degrees = yaw_degrees;
        self
    }

    pub fn pitch_degrees(mut self, pitch_degrees: f32) -> Self {
        self.pitch_degrees = pitch_degrees;
        self
    }

    pub fn rotate_yaw_pitch(&mut self, yaw_degrees: f32, pitch_degrees: f32) {
        self.pitch_degrees = (self.pitch_degrees + pitch_degrees).clamp(-90.0, 90.0);
        self.yaw_degrees = (self.yaw_degrees + yaw_degrees) % 720_f32;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
        self.yaw_degrees = yaw.to_degrees();
        self.pitch_degrees = pitch.to_degrees();
    }
}

impl RigDriver for YawPitch {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: params.parent.translation,
            rotation: Quat::from_euler(
                EulerRot::YXZ,
                self.yaw_degrees.to_radians(),
                self.pitch_degrees.to_radians(),
                0.0,
            ),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
