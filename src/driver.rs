use crate::{rig::RigUpdateParams, transform::Transform};

pub trait RigDriver: std::any::Any + std::fmt::Debug {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams) -> Transform;

    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
