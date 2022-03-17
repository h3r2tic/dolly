use crate::{handedness::Handedness, rig::RigUpdateParams, transform::Transform};

pub trait RigDriverTraits<H: Handedness + 'static>:
    RigDriver<H> + Sync + Send + std::any::Any + std::fmt::Debug
{
    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait RigDriver<H: Handedness + 'static>: std::any::Any + std::fmt::Debug {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H>;
}

impl<H: Handedness + 'static, T> RigDriverTraits<H> for T
where
    T: RigDriver<H> + std::any::Any + Sync + Send + std::fmt::Debug,
{
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
