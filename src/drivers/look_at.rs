use glam::{Mat3, Quat, Vec3};

use crate::{
    driver::RigDriver,
    rig::RigUpdateParams,
    transform::Transform,
    util::{ExpSmoothed, ExpSmoothingParams},
};

pub struct LookAt {
    pub smoothness: f32,
    pub target: Vec3,
    output_offset_scale: f32,
    smoothed_target: ExpSmoothed<Vec3>,
}

impl LookAt {
    pub fn new(target: Vec3) -> Self {
        Self {
            smoothness: 0.0,
            output_offset_scale: 1.0,
            target,
            smoothed_target: Default::default(),
        }
    }

    pub fn smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    pub fn predictive(mut self, scale: f32) -> Self {
        self.output_offset_scale = -scale;
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
                dt: params.dt,
            },
        );

        let rotation = (target - params.parent.translation)
            .try_normalize()
            .and_then(|forward| {
                let right = forward.cross(Vec3::Y).try_normalize()?;
                let up = right.cross(forward);
                Some(Quat::from_mat3(&Mat3::from_cols(right, up, -forward)))
            })
            .unwrap_or_default();

        Transform {
            translation: params.parent.translation,
            rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
