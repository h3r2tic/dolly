use glam::Quat;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

/// Locks/constrains the rotation of the camera to one or more axes
#[derive(Debug)]
pub struct LockRotation {
    x: bool,
    y: bool,
    z: bool,
}

impl LockRotation {
    pub fn new() -> Self {
        Self { x:false, y:false, z:false }
    }
    pub fn from(x: bool,y :bool, z:bool) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> Self{
        Self { x: true, y: self.y, z: self.z}
    }
    pub fn y(&self) -> Self{
        Self { x: self.x, y: true, z: self.z}
    }
    pub fn z(&self) -> Self{
        Self { x: self.x, y: self.y, z: true}
    }
}

impl RigDriver for LockRotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let rot = params.parent.rotation;
        let mut delta = Quat::IDENTITY;
        if self.x {
            //delta *= Quat::from_xyzw(rot.x, 0., rot.z, rot.w).normalize();
            delta *= rot;
        }
        if self.y {
            delta *= Quat::from_xyzw(0., rot.y, 0., rot.w).normalize();
        }
        if self.z {
            delta *= rot;
            //delta *= Quat::from_xyzw(0., 0., rot.z, rot.w).normalize();
        }
        
        Transform {
            position: params.parent.position,
            rotation: delta,
        }
    }
}
