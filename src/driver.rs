use crate::{rig::RigUpdateParams, transform::Transform};

pub trait RigDriver: std::any::Any {
    fn update(&mut self, params: RigUpdateParams) -> Transform;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
