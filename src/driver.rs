use crate::{handedness::Handedness, rig::RigUpdateParams, transform::Transform};

pub trait RigDriverTraits<H: Handedness>:
    RigDriver<H> + Sync + Send + std::any::Any + std::fmt::Debug
{
    /// Returns `self` as `&dyn Any`
    fn as_any(&self) -> &dyn std::any::Any;

    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait RigDriver<H: Handedness>: std::any::Any + std::fmt::Debug {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H>;
}

impl<H: Handedness, T> RigDriverTraits<H> for T
where
    T: RigDriver<H> + std::any::Any + Sync + Send + std::fmt::Debug,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
