# 🎥 dolly

[![Crates.io](https://img.shields.io/crates/v/dolly.svg)](https://crates.io/crates/dolly)
[![Docs](https://docs.rs/dolly/badge.svg)](https://docs.rs/dolly)

Combine simple building blocks to create smooth cameras: first-person, chase, orbit, look-at, you name it!

Camera rigs made with `dolly` are engine-agnostic, and only provide camera positioning. Optical and rendering parameters such as field of view and clipping planes can be built on top, and are not within the scope of this crate.

While cameras are a complex topic in gamedev, this crate only provides the basics, aiming at small games and tools.

## Example

![orbit camera example](https://oxn9dw.db.files.1drv.com/y4m_cv-3paMLNbYajhGOWPvn172gkHhrOUzmaPXUo8JgZgiFYrygLrt9IrUXXcsoTNf2naYm4Qg-V5JzRSRgwK3-u0bj348uKXUYq8k6ntGWiYpDPMl61P-v42YSFL7lr-IMedLAGheJP54tRBzElRwz4bSzxHdHPJIkXYuBbzmAFEhbX1yHl8uHTGedeHUgnJj0qbMI7fgH9VXNUKzUVaZpw/orbit.gif?download&psid=1)

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