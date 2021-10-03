use crate::{driver::RigDriver, rig::RigUpdateParams, transform::Transform};

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z
}

/// Locks/constrains the position of the camera to one or more axes
#[derive(Debug)]
pub struct LockPosition {
    pub axes: &'static [Axis],
}

impl LockPosition {
    pub fn new(axes: &'static [Axis]) -> Self {
        Self { axes }
    }
}

impl RigDriver for LockPosition {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let mut delta_pos = params.parent.position;
        if self.axes.iter().any(|axis| axis == &Axis::X) {
            delta_pos.x = 0.
        }
        if self.axes.iter().any(|axis| axis == &Axis::Y) {
            delta_pos.x = 0.
        }
        if self.axes.iter().any(|axis| axis == &Axis::Z) {
            delta_pos.x = 0.
        }
        Transform {
            position: delta_pos,
            rotation: params.parent.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
