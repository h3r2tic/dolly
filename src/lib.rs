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

pub struct YawPitch {
    pub yaw_degrees: f32,
    pub pitch_degrees: f32,
    pub position: Vec3,
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
            position: Vec3::ZERO,
        }
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
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

    pub fn translate(&mut self, move_vec: Vec3) {
        self.position += move_vec;
    }
}

impl RigDriver for YawPitch {
    fn update(&mut self, _: RigUpdateParams) -> Transform {
        Transform {
            translation: self.position,
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
    pub interp_transform: Option<Transform>,
}

impl Default for Smooth {
    fn default() -> Self {
        Self::new()
    }
}

impl Smooth {
    pub fn new() -> Self {
        Self {
            move_smoothness: 1.0,
            look_smoothness: 1.0,
            interp_transform: None,
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
}

impl RigDriver for Smooth {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let rot_interp = 1.0 - (-30.0 * params.dt / self.look_smoothness.max(1e-5)).exp();
        let pos_interp = 1.0 - (-16.0 * params.dt / self.move_smoothness.max(1e-5)).exp();

        let prev_transform = self.interp_transform.unwrap_or(*params.parent);
        let transform = Transform {
            rotation: prev_transform
                .rotation
                .slerp(params.parent.rotation, rot_interp)
                .normalize(),

            translation: prev_transform
                .translation
                .lerp(params.parent.translation, pos_interp),
        };

        self.interp_transform = Some(transform);
        transform
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Arm {
    pub length: f32,
}

impl Arm {
    pub fn new(length: f32) -> Self {
        Self { length }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            rotation: params.parent.rotation,
            translation: params.parent.translation + params.parent.rotation * Vec3::Z * self.length,
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
