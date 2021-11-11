use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

/// Locks/constrains the position of the camera to one or more axes
#[derive(Debug)]
pub struct LockPosition {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl LockPosition {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }
    pub fn from(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self, x: f32) -> Self {
        Self {
            x: Some(x),
            y: self.y,
            z: self.z,
        }
    }
    pub fn y(&self, y: f32) -> Self {
        Self {
            x: self.x,
            y: Some(y),
            z: self.z,
        }
    }
    pub fn z(&self, z: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: Some(z),
        }
    }
}

impl Default for LockPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl RigDriver for LockPosition {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let mut delta_pos = params.parent.position;
        delta_pos.x = self.x.unwrap_or(delta_pos.x);
        delta_pos.y = self.y.unwrap_or(delta_pos.y);
        delta_pos.z = self.z.unwrap_or(delta_pos.z);
        Transform {
            position: delta_pos,
            rotation: params.parent.rotation,
        }
    }
}
