# 🎥 dolly

[![Crates.io](https://img.shields.io/crates/v/dolly.svg)](https://crates.io/crates/dolly)
[![Docs](https://docs.rs/dolly/badge.svg)](https://docs.rs/dolly)
[![Rust 1.68.2](https://img.shields.io/badge/Rust-1.68.2-blue?logo=rust)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1682-2023-03-28)

Combine simple building blocks to create smooth cameras: first-person, chase, orbit, look-at, you name it!

Camera rigs made with `dolly` are engine-agnostic, and only provide camera positioning. Optical and rendering parameters such as field of view and clipping planes can be built on top, and are not within the scope of this crate.

While cameras are a complex topic in gamedev, this crate only provides the basics, aiming at small games and tools.

## Examples

<https://user-images.githubusercontent.com/16522064/125960266-fc96b302-6d6b-4976-b38c-b6f4fdb8e09b.mp4>

```rust
let mut camera: CameraRig = CameraRig::builder()
    .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
    .with(Smooth::new_rotation(1.5))
    .with(Arm::new(Vec3::Z * 4.0))
    .build();

// ...

let camera_driver = camera.driver_mut::<YawPitch>();
if keyboard.was_just_pressed(VirtualKeyCode::Z) {
    camera_driver.rotate_yaw_pitch(-90.0, 0.0);
}
if keyboard.was_just_pressed(VirtualKeyCode::X) {
    camera_driver.rotate_yaw_pitch(90.0, 0.0);
}

camera.update(time_delta_seconds);
```

---

<https://user-images.githubusercontent.com/16522064/125960227-7ee05c04-f47a-4c32-b494-cc36dc70ab63.mp4>

```rust
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
```

---

<https://user-images.githubusercontent.com/16522064/125986386-60cb9d26-06a2-4d3f-9377-56c982fbc7f9.mp4>

```rust
let mut camera: CameraRig = CameraRig::builder()
    .with(Position::new(Vec3::Y * 3.0))
    .with(LookAt::new(car.position))
    .build();

// ...

camera.driver_mut::<LookAt>().target = car.position.into();
camera.update(time_delta_seconds);
```

---

<https://user-images.githubusercontent.com/16522064/125986405-a06f6572-702a-4c1a-a6c7-edf5ba2ed815.mp4>

```rust
let mut camera: CameraRig = CameraRig::builder()
    .with(Position::new(Vec3::Y))
    .with(YawPitch::new())
    .with(Smooth::new_position_rotation(1.0, 1.0))
    .build();

// ...

let move_vec = camera.transform.rotation
    * Vec3::new(input["move_right"], input["move_up"], -input["move_fwd"])
        .clamp_length_max(1.0)
    * 10.0f32.powf(input["boost"]);

camera
    .driver_mut::<YawPitch>()
    .rotate_yaw_pitch(-0.3 * mouse.delta.x, -0.3 * mouse.delta.y);
camera
    .driver_mut::<Position>()
    .translate(move_vec * time_delta_seconds * 10.0);
camera.update(time_delta_seconds);
```
