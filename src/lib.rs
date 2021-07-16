pub mod driver;
pub mod drivers;
pub mod prelude;
pub mod rig;
pub mod transform;

mod util;

pub use glam;

#[test]
fn test() {
    use crate::prelude::*;

    let mut camera = CameraRig::builder()
        .with(Positional::new(glam::Vec3::ZERO))
        .with(YawPitch::new())
        .with(Smooth::new_move_look(1.0, 1.0))
        .build();

    // ...

    camera.driver_mut::<YawPitch>().rotate_yaw_pitch(10.0, 0.0);
    camera.update(1.0 / 60.0);
}
