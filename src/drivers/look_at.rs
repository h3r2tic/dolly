use std::marker::PhantomData;

use glam::Vec3;

use crate::{
    driver::RigDriver,
    handedness::Handedness,
    rig::RigUpdateParams,
    transform::Transform,
    util::{look_at, ExpSmoothed, ExpSmoothingParams},
};

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, and made to look ahead of it.
#[derive(Debug)]
pub struct LookAt {
    /// Exponential smoothing factor
    pub smoothness: f32,

    /// The world-space position to look at
    pub target: mint::Point3<f32>,

    // The scale with which smoothing should be applied to the target position
    output_offset_scale: f32,

    smoothed_target: ExpSmoothed<Vec3>,
}

impl LookAt {
    pub fn new<P>(target: P) -> Self
    where
        P: Into<mint::Point3<f32>>,
    {
        let target = target.into();

        Self {
            smoothness: 0.0,
            output_offset_scale: 1.0,
            target,
            smoothed_target: Default::default(),
        }
    }

    /// Set the exponential smoothing factor for target position tracking.
    pub fn tracking_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    /// Reverse target position smoothing, causing the camera to look ahead of it.
    /// This can then be chained with [`Smooth`], to create
    /// a camera that smoothly follows an object, but doesn't lag far behind it.
    ///
    /// [`Smooth`]: struct.Smooth.html
    pub fn tracking_predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive { -1.0 } else { 1.0 };
        self
    }
}

impl<H: Handedness> RigDriver<H> for LookAt {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        let other: Vec3 = self.target.into();

        let target = self.smoothed_target.exp_smooth_towards(
            &other,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        let parent_position: Vec3 = From::from(params.parent.position);
        let rotation = look_at::<H, _, _>(target - parent_position);

        Transform {
            position: params.parent.position,
            rotation,
            phantom: PhantomData,
        }
    }
}
