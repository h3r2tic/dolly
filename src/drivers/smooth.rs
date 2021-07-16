use glam::{Quat, Vec3};

use crate::{
    driver::RigDriver,
    rig::RigUpdateParams,
    transform::Transform,
    util::{ExpSmoothed, ExpSmoothingParams},
};

pub struct Smooth {
    pub move_smoothness: f32,
    pub look_smoothness: f32,
    output_offset_scale: f32,
    smoothed_translation: ExpSmoothed<Vec3>,
    smoothed_rotation: ExpSmoothed<Quat>,
}

impl Default for Smooth {
    fn default() -> Self {
        Self {
            move_smoothness: 1.0,
            look_smoothness: 1.0,
            output_offset_scale: 1.0,
            smoothed_translation: Default::default(),
            smoothed_rotation: Default::default(),
        }
    }
}

impl Smooth {
    pub fn new_move(move_smoothness: f32) -> Self {
        Self {
            move_smoothness,
            look_smoothness: 0.0,
            ..Default::default()
        }
    }

    pub fn new_look(look_smoothness: f32) -> Self {
        Self {
            look_smoothness,
            move_smoothness: 0.0,
            ..Default::default()
        }
    }

    pub fn new_move_look(move_smoothness: f32, look_smoothness: f32) -> Self {
        Self {
            move_smoothness,
            look_smoothness,
            ..Default::default()
        }
    }

    pub fn move_smoothness(mut self, move_smoothness: f32) -> Self {
        self.move_smoothness = move_smoothness;
        self
    }

    pub fn look_smoothness(mut self, look_smoothness: f32) -> Self {
        self.look_smoothness = look_smoothness;
        self
    }

    pub fn predictive(mut self, scale: f32) -> Self {
        self.output_offset_scale = -scale;
        self
    }
}

impl RigDriver for Smooth {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let translation = self.smoothed_translation.exp_smooth_towards(
            &params.parent.translation,
            ExpSmoothingParams {
                smoothness: self.move_smoothness,
                output_offset_scale: self.output_offset_scale,
                dt: params.dt,
            },
        );

        let rotation = self.smoothed_rotation.exp_smooth_towards(
            &params.parent.rotation,
            ExpSmoothingParams {
                smoothness: self.look_smoothness,
                output_offset_scale: self.output_offset_scale,
                dt: params.dt,
            },
        );

        Transform {
            translation,
            rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
