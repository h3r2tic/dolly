pub use crate::{drivers::*, handedness::*, rig::CameraRig};

#[test]
fn orbit_example_compile_test() {
    use glam::Vec3;

    let mut camera: CameraRig = CameraRig::builder()
        .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
        .with(Smooth::new_rotation(1.5))
        .with(Arm::new(Vec3::Z * 4.0))
        .build();

    // ...

    fn was_key_pressed(_: char) -> bool {
        true
    }

    let camera_driver = camera.driver_mut::<YawPitch>();
    if was_key_pressed('Z') {
        camera_driver.rotate_yaw_pitch(-90.0, 0.0);
    }
    if was_key_pressed('X') {
        camera_driver.rotate_yaw_pitch(90.0, 0.0);
    }

    let time_delta_seconds = 1.0 / 60.0;
    camera.update(time_delta_seconds);
}

#[test]
fn follow_example_compile_test() {
    use glam::{Quat, Vec3};

    #[derive(Default)]
    struct Car {
        position: Vec3,
        rotation: Quat,
    }

    let car = Car::default();

    let mut camera: CameraRig = CameraRig::builder()
        .with(Position::new(car.position))
        .with(Rotation::new(car.rotation))
        .with(Smooth::new_position(1.25).predictive(true))
        .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
        .with(Smooth::new_position(2.5))
        .with(
            LookAt::new(car.position + Vec3::Y)
                .tracking_smoothness(1.25)
                .tracking_predictive(true),
        )
        .build();

    // ...

    camera.driver_mut::<Position>().position = car.position.into();
    camera.driver_mut::<Rotation>().rotation = car.rotation.into();
    camera.driver_mut::<LookAt>().target = (car.position + Vec3::Y).into();
}

#[test]
fn lookat_example_compile_test() {
    use glam::Vec3;

    let mut camera: CameraRig = CameraRig::builder()
        .with(Position::new(Vec3::Y * 3.0))
        .with(LookAt::new(Vec3::ZERO))
        .build();

    // ...

    let time_delta_seconds = 1.0 / 60.0;
    camera.driver_mut::<LookAt>().target = Vec3::X.into();
    camera.update(time_delta_seconds);
}

#[test]
fn free_example_compile_test() {
    use glam::Vec3;

    let mut camera: CameraRig = CameraRig::builder()
        .with(Position::new(Vec3::Y))
        .with(YawPitch::new())
        .with(Smooth::new_position_rotation(1.0, 1.0))
        .build();

    // ...

    let move_vec = Vec3::new(0.1, 0.2, 0.3);
    let time_delta_seconds = 1.0 / 60.0;

    camera.driver_mut::<YawPitch>().rotate_yaw_pitch(-0.3, -0.2);
    camera
        .driver_mut::<Position>()
        .translate(move_vec * time_delta_seconds);
    camera.update(time_delta_seconds);
}
