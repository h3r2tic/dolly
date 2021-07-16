use glam::{Quat, Vec3};

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
        Quat::lerp(self.normalize(), other.normalize(), t).normalize()
    }
}

pub(crate) struct ExpSmoothingParams {
    pub smoothness: f32,
    pub output_offset_scale: f32,
    pub dt: f32,
}

#[derive(Default)]
pub(crate) struct ExpSmoothed<T: Interpolate + Copy>(Option<T>);

impl<T: Interpolate + Copy> ExpSmoothed<T> {
    pub(crate) fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        let interp_t = 1.0 - (-8.0 * params.dt / params.smoothness.max(1e-5)).exp();

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
