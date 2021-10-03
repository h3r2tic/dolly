use glam::Quat;

use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

use super::Axis;

/// Locks/constrains the rotation of the camera to one or more axes
#[derive(Debug)]
pub struct LockRotation {
    pub axes: &'static [Axis],
}

impl LockRotation {
    pub fn new(axes: &'static [Axis]) -> Self {
        Self { axes }
    }
}

impl RigDriver for LockRotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let (mut euler, a) = params.parent.rotation.to_axis_angle();
        if self.axes.iter().any(|axis| axis == &Axis::X) {
            euler.x = 0.;
        }
        if self.axes.iter().any(|axis| axis == &Axis::Y) {
            euler.y = 0.;
        }
        if self.axes.iter().any(|axis| axis == &Axis::Z) {
            euler.z = 0.;
        }
        Transform {
            position: params.parent.position,
            rotation: Quat::from_axis_angle(euler, a),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
