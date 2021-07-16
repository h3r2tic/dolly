use glam::{EulerRot, Quat, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl Transform {
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn into_translation_rotation(self) -> (Vec3, Quat) {
        (self.translation, self.rotation)
    }

    pub const IDENTITY: Transform = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
}

pub struct CameraRig {
    pub drivers: Vec<Box<dyn RigDriver>>,
    pub transform: Transform,
}

struct RigUpdateToken;

pub struct RigUpdateParams<'a> {
    pub parent: &'a Transform,
    pub dt: f32,
    _token: RigUpdateToken,
}

impl CameraRig {
    pub fn driver_mut<T: RigDriver>(&mut self) -> &mut T {
        for driver in &mut self.drivers {
            if let Some(driver) = driver.as_mut().as_any_mut().downcast_mut::<T>() {
                return driver;
            }
        }

        panic!();
    }
}

pub trait RigDriver: std::any::Any {
    fn update(&mut self, params: RigUpdateParams) -> Transform;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Default)]
pub struct Positional {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Positional {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
    }

    pub fn set_position_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }
}

impl RigDriver for Positional {
    fn update(&mut self, _: RigUpdateParams) -> Transform {
        Transform {
            translation: self.position,
            rotation: self.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct YawPitch {
    pub yaw_degrees: f32,
    pub pitch_degrees: f32,
}

impl Default for YawPitch {
    fn default() -> Self {
        Self::new()
    }
}

impl YawPitch {
    pub fn new() -> Self {
        Self {
            yaw_degrees: 0.0,
            pitch_degrees: 0.0,
        }
    }

    pub fn rotation(mut self, rotation: Quat) -> Self {
        let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
        self.yaw_degrees = yaw.to_degrees();
        self.pitch_degrees = pitch.to_degrees();
        self
    }

    pub fn yaw_degrees(mut self, yaw_degrees: f32) -> Self {
        self.yaw_degrees = yaw_degrees;
        self
    }

    pub fn pitch_degrees(mut self, pitch_degrees: f32) -> Self {
        self.pitch_degrees = pitch_degrees;
        self
    }

    pub fn rotate_yaw_pitch(&mut self, yaw_degrees: f32, pitch_degrees: f32) {
        self.pitch_degrees = (self.pitch_degrees + pitch_degrees).clamp(-90.0, 90.0);
        self.yaw_degrees = (self.yaw_degrees + yaw_degrees) % 720_f32;
    }
}

impl RigDriver for YawPitch {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: params.parent.translation,
            rotation: Quat::from_euler(
                EulerRot::YXZ,
                self.yaw_degrees.to_radians(),
                self.pitch_degrees.to_radians(),
                0.0,
            ),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

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

        Transform {
            translation: params.parent.translation,
            rotation: Quat::from_mat4(&glam::Mat4::look_at_rh(
                params.parent.translation,
                target,
                Vec3::Y,
            ))
            .conjugate()
            .normalize(),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Arm {
    pub offset: Vec3,
}

impl Arm {
    pub fn new(offset: Vec3) -> Self {
        Self { offset }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            rotation: params.parent.rotation,
            translation: params.parent.translation + params.parent.rotation * self.offset,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl CameraRig {
    pub fn update(&mut self, dt: f32) -> Transform {
        let mut parent_transform = Transform::IDENTITY;

        for driver in self.drivers.iter_mut() {
            let transform = driver.update(RigUpdateParams {
                parent: &parent_transform,
                dt,
                _token: RigUpdateToken,
            });

            parent_transform = transform;
        }

        self.transform = parent_transform;
        self.transform
    }
}

pub struct CameraRigBuilder {
    drivers: Vec<Box<dyn RigDriver>>,
}

impl CameraRig {
    pub fn builder() -> CameraRigBuilder {
        CameraRigBuilder {
            drivers: Default::default(),
        }
    }
}

impl CameraRigBuilder {
    pub fn with(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    pub fn build(self) -> CameraRig {
        let mut rig = CameraRig {
            drivers: self.drivers,
            transform: Transform::IDENTITY,
        };

        rig.update(0.0);
        rig
    }
}

trait Interpolate {
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

struct ExpSmoothingParams {
    smoothness: f32,
    output_offset_scale: f32,
    dt: f32,
}

#[derive(Default)]
struct ExpSmoothed<T: Interpolate + Copy>(Option<T>);

impl<T: Interpolate + Copy> ExpSmoothed<T> {
    fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        let interp_t = 1.0 - (-8.0 * params.dt / params.smoothness.max(1e-5)).exp();

        let prev = self.0.unwrap_or(*other);
        let smooth = prev.interpolate(*other, interp_t);

        self.0 = Some(smooth);

        if params.output_offset_scale != 1.0 {
            Interpolate::interpolate(*other, smooth, params.output_offset_scale)
        } else {
            smooth
        }
    }
}

#[test]
fn test() {
    let mut camera = CameraRig::builder()
        .with(Positional::new(Vec3::ZERO))
        .with(YawPitch::new())
        .with(Smooth::new_move_look(1.0, 1.0))
        .build();

    // ...

    camera.driver_mut::<YawPitch>().rotate_yaw_pitch(10.0, 0.0);
    camera.update(1.0 / 60.0);
}
