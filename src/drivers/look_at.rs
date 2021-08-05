use glam::{Mat3, Quat, Vec3};

use crate::{
    driver::RigDriver,
    rig::RigUpdateParams,
    transform::Transform,
    util::{ExpSmoothed, ExpSmoothingParams},
};

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, and made to look ahead of it.
#[derive(Debug)]
pub struct LookAt {
    /// Exponential smoothing factor
    pub smoothness: f32,

    /// The world-space position to look at
    pub target: Vec3,

    // The scale with which smoothing should be applied to the target position
    output_offset_scale: f32,

    smoothed_target: ExpSmoothed<Vec3>,
}

impl LookAt {
    ///
    pub fn new(target: Vec3) -> Self {
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

impl RigDriver for LookAt {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let target = self.smoothed_target.exp_smooth_towards(
            &self.target,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        let rotation = (target - params.parent.position)
            .try_normalize()
            .and_then(|forward| {
                let right = forward.cross(Vec3::Y).try_normalize()?;
                let up = right.cross(forward);
                Some(Quat::from_mat3(&Mat3::from_cols(right, up, -forward)))
            })
            .unwrap_or_default();

        Transform {
            position: params.parent.position,
            rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
