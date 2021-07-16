# 🎥 dolly

Create smooth first-person, chase, orbit and other camera types out of simple building blocks.

Camera rigs made with `dolly` are engine-agnostic, and only provide camera positioning. Optical and rendering parameters such as field of view and clipping planes can be built on top, and are not within the scope of this crate.

While cameras are a complex topic in AAA productions, this crate only provides basics, aiming at simple games and tools.

## Examples

![follow camera](media/follow.mp4)

```rust
let mut camera = CameraRig::builder()
    .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
    .with(Smooth::new_look(1.5))
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

camera.update(delta_seconds);
```

---

![orbit camera](media/orbit.mp4)

```rust
let mut camera = CameraRig::builder()
    .with(Positional::new(car.position))
    .with(Smooth::new_move(1.25).predictive(1.0))
    .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
    .with(Smooth::new_move(2.5))
    .with(
        LookAt::new(car.position + Vec3::Y)
            .smoothness(1.25)
            .predictive(1.0),
    )
    .build();

// ...

camera
    .driver_mut::<Positional>()
    .set_position_rotation(car.position, car.rotation);
camera.driver_mut::<LookAt>().target = car.position + Vec3::Y;

camera.update(delta_seconds);
```