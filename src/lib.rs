//use std::sync::mpsc;

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
    pub fn new() -> CameraRigBuilder {
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

/*pub trait RigElement {
    fn update(&mut self, dt: f32);
    fn transform(&self) -> Transform;
}

pub struct FirstPersonCameraInput {
    pub move_vec: Vec3,
    pub yaw_delta: f32,
    pub pitch_delta: f32,
}

impl FirstPersonCameraInput {
    pub fn relative_to(self, transform: &Transform) -> Self {
        FirstPersonCameraInput {
            move_vec: transform.rotation * self.move_vec,
            yaw_delta: self.yaw_delta,
            pitch_delta: self.pitch_delta,
        }
    }
}

pub struct FirstPerson {
    // Degrees
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub move_speed: f32,

    transform: Transform,
    input_receiver: mpsc::Receiver<FirstPersonCameraInput>,
}

impl FirstPerson {
    pub fn from_transform(transform: Transform) -> (Self, mpsc::Sender<FirstPersonCameraInput>) {
        let (tx, rx) = mpsc::channel();

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        (
            Self {
                yaw: yaw.to_degrees(),
                pitch: pitch.to_degrees(),
                roll: roll.to_degrees(),
                move_speed: 10.0,
                transform,
                input_receiver: rx,
            },
            tx,
        )
    }

    pub fn from_position_rotation(
        position: Vec3,
        rotation: Quat,
    ) -> (Self, mpsc::Sender<FirstPersonCameraInput>) {
        Self::from_transform(Transform {
            translation: position,
            rotation,
        })
    }

    pub fn from_position_yaw_pitch_roll(
        position: Vec3,
        yaw_degrees: f32,
        pitch_degrees: f32,
        roll_degrees: f32,
    ) -> (Self, mpsc::Sender<FirstPersonCameraInput>) {
        let (tx, rx) = mpsc::channel();
        (
            Self {
                yaw: yaw_degrees,
                pitch: pitch_degrees,
                roll: roll_degrees,
                move_speed: 10.0,
                transform: Transform {
                    translation: position,
                    rotation: Quat::from_euler(
                        EulerRot::YXZ,
                        yaw_degrees.to_radians(),
                        pitch_degrees.to_radians(),
                        roll_degrees.to_radians(),
                    ),
                },
                input_receiver: rx,
            },
            tx,
        )
    }
}

impl RigElement for FirstPerson {
    fn update(&mut self, dt: f32) {
        let input = self.input_receiver.try_recv().unwrap();

        self.transform.translation += input.move_vec.clamp_length_max(1.0) * (self.move_speed * dt);

        self.pitch = (self.pitch + input.pitch_delta).clamp(-90.0, 90.0);
        self.yaw = (self.yaw + input.yaw_delta) % 720_f32;

        self.transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            self.yaw.to_radians(),
            self.pitch.to_radians(),
            self.roll.to_radians(),
        );
    }

    fn transform(&self) -> Transform {
        self.transform
    }
}

#[derive(Clone, Copy)]
pub struct InterpolatedDesc {
    pub move_smoothness: f32,
    pub look_smoothness: f32,
}

impl Default for InterpolatedDesc {
    fn default() -> Self {
        Self {
            move_smoothness: 1.0,
            look_smoothness: 1.0,
        }
    }
}

pub struct Interpolated<T: RigElement> {
    inner: T,
    transform: Transform,
    desc: InterpolatedDesc,
}

impl<T: RigElement> RigElement for Interpolated<T> {
    fn update(&mut self, dt: f32) {
        self.inner.update(dt);
        let inner_transform = self.inner.transform();

        let rot_interp = 1.0 - (-1.0 * dt / self.desc.look_smoothness.max(1e-5)).exp();
        let pos_interp = 1.0 - (-1.0 * dt / self.desc.move_smoothness.max(1e-5)).exp();

        self.transform.rotation = self
            .transform
            .rotation
            .slerp(inner_transform.rotation, rot_interp)
            .normalize();

        self.transform.translation = self
            .transform
            .translation
            .lerp(inner_transform.translation, pos_interp);
    }

    fn transform(&self) -> Transform {
        self.transform
    }
}

pub trait RigInterpolated: RigElement + Sized {
    fn interpolated(self, desc: InterpolatedDesc) -> Interpolated<Self>;
}

impl<T: RigElement> RigInterpolated for T {
    fn interpolated(self, desc: InterpolatedDesc) -> Interpolated<T> {
        let inner_transform = self.transform();
        Interpolated {
            inner: self,
            transform: inner_transform,
            desc,
        }
    }
}

pub struct Arm<T: RigElement> {
    inner: T,
    arm_length: f32,
}

impl<T: RigElement> RigElement for Arm<T> {
    fn update(&mut self, dt: f32) {
        self.inner.update(dt);
    }

    fn transform(&self) -> Transform {
        let inner_transform = self.inner.transform();

        Transform {
            rotation: inner_transform.rotation,
            translation: inner_transform.translation
                + inner_transform.rotation * Vec3::Z * self.arm_length,
        }
    }
}

pub trait RigArm: RigElement + Sized {
    fn arm(self, arm_length: f32) -> Arm<Self>;
}

impl<T: RigElement> RigArm for T {
    fn arm(self, arm_length: f32) -> Arm<T> {
        Arm {
            inner: self,
            arm_length,
        }
    }
}

#[test]
fn test() {
    let (camera, camera_input) = FirstPerson::from_position_rotation(Vec3::ZERO, Quat::IDENTITY);
    let mut camera = camera.interpolated(InterpolatedDesc::default());

    camera_input
        .send(FirstPersonCameraInput {
            move_vec: Vec3::ZERO,
            yaw_delta: 0.0,
            pitch_delta: 0.0,
        })
        .unwrap();
    camera.update(1.0 / 60.0);

    let mut camera_rig = CameraRig::new()
        .with(YawPitch::new())
        .with(Smooth::new())
        .build();
    camera_rig.driver_mut::<YawPitch>();
}
*/
