# 🎥 dolly

Combine simple building blocks to create smooth cameras: first-person, chase, orbit, look-at, you name it!

Camera rigs made with `dolly` are engine-agnostic, and only provide camera positioning. Optical and rendering parameters such as field of view and clipping planes can be built on top, and are not within the scope of this crate.

While cameras are a complex topic in gamedev, this crate only provides the basics, aiming at small games and tools.

## Examples

https://user-images.githubusercontent.com/16522064/125960266-fc96b302-6d6b-4976-b38c-b6f4fdb8e09b.mp4

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

camera.update(time_delta_seconds);
```

---

https://user-images.githubusercontent.com/16522064/125960227-7ee05c04-f47a-4c32-b494-cc36dc70ab63.mp4

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
camera.update(time_delta_seconds);
```

---

https://user-images.githubusercontent.com/16522064/125986386-60cb9d26-06a2-4d3f-9377-56c982fbc7f9.mp4

```rust
let mut camera = CameraRig::builder()
    .with(Positional::new(Vec3::Y * 3.0))
    .with(LookAt::new(car.position))
    .build();

// ...

camera.driver_mut::<LookAt>().target = car.position;
camera.update(time_delta_seconds);
```

---

https://user-images.githubusercontent.com/16522064/125986405-a06f6572-702a-4c1a-a6c7-edf5ba2ed815.mp4

```rust
let mut camera = CameraRig::builder()
    .with(Positional::new(Vec3::Y))
    .with(YawPitch::new())
    .with(Smooth::new_move_look(1.0, 1.0))
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
    .driver_mut::<Positional>()
    .translate(move_vec * time_delta_seconds * 10.0);
camera.update(time_delta_seconds);
```
