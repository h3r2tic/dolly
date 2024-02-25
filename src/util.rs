use glam::{Mat3, Quat, Vec3};

use crate::prelude::Handedness;

pub(crate) trait Interpolate {
    fn interpolate(self, other: Self, t: f32) -> Self;
}

impl Interpolate for Vec3 {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Vec3::lerp(self, other, t)
    }
}

impl Interpolate for Quat {
    fn interpolate(self, other: Self, t: f32) -> Self {
        // Technically should be a `slerp` for framerate independence, but the latter
        // will rotate in the negative direction when interpolating a 180..360 degree rotation
        // to the 0..180 range. See the comment about `yaw_degrees` in `YawPitch` for more details.
        Quat::lerp(self.normalize(), other.normalize(), t).normalize()
    }
}

pub(crate) struct ExpSmoothingParams {
    pub smoothness: f32,
    pub output_offset_scale: f32,
    pub delta_time_seconds: f32,
}

#[derive(Default, Debug)]
pub(crate) struct ExpSmoothed<T: Interpolate + Copy + std::fmt::Debug>(Option<T>);

impl<T: Interpolate + Copy + std::fmt::Debug> ExpSmoothed<T> {
    pub(crate) fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        // An ad-hoc multiplier to make default smoothness parameters
        // produce good-looking results.
        const SMOOTHNESS_MULT: f32 = 8.0;

        // Calculate the exponential blending based on frame time
        let interp_t = 1.0
            - (-SMOOTHNESS_MULT * params.delta_time_seconds / params.smoothness.max(1e-5)).exp();

        let prev = self.0.unwrap_or(*other);
        let smooth = prev.interpolate(*other, interp_t);

        self.0 = Some(smooth);

        #[allow(clippy::float_cmp)]
        if params.output_offset_scale != 1.0 {
            Interpolate::interpolate(*other, smooth, params.output_offset_scale)
        } else {
            smooth
        }
    }
}

pub fn look_at<H: Handedness, V, Q>(forward: V) -> Q
where
    V: Into<mint::Vector3<f32>>,
    Q: From<mint::Quaternion<f32>>,
{
    let forward: Vec3 = forward.into().into();

    let result = forward
        .try_normalize()
        .and_then(|forward| {
            let right =
                H::right_from_up_and_forward::<Vec3, Vec3>(Vec3::Y, forward).try_normalize()?;
            let up = H::up_from_right_and_forward(right, forward);
            Some(Quat::from_mat3(&Mat3::from_cols(
                right,
                up,
                forward * H::FORWARD_Z_SIGN,
            )))
        })
        .unwrap_or_default();

    From::from(result.into())
}
