// Based on https://github.com/not-fl3/macroquad/blob/97a99d00155cb7531f4432a2eb5f3c587e22f9b3/examples/3d.rs

use dolly::{driver::RigDriver, prelude::*};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Follow(CameraRig);

impl RigDriver for Follow {
    fn update(&mut self, params: dolly::rig::RigUpdateParams) -> dolly::transform::Transform {
        self.0.update(params.delta_time_seconds)
    }
}

impl Follow {
    pub fn from_transform(transform: dolly::transform::Transform) -> Self {
        Self(
            CameraRig::builder()
                .with(Position::new(transform.position))
                .with(Rotation::new(transform.rotation))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(dolly::glam::Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(transform.position + dolly::glam::Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        )
    }

    pub fn follow(
        &mut self,
        position: dolly::glam::Vec3,
        rotation: dolly::glam::Quat,
        target: dolly::glam::Vec3,
    ) {
        self.0.driver_mut::<Position>().position = position;
        self.0.driver_mut::<Rotation>().rotation = rotation;
        self.0.driver_mut::<LookAt>().target = target;
    }
}

#[macroquad::main("dolly example")]
async fn main() {
    // Create a smoothed orbit camera
    let mut camera = CameraRig::builder()
        .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
        .with(Smooth::new_rotation(1.5))
        .with(Arm::new(dolly::glam::Vec3::Z * 8.0))
        .build();

    loop {
        // Handle camera input
        if is_key_pressed(KeyCode::Z) {
            camera.driver_mut::<YawPitch>().rotate_yaw_pitch(-90.0, 0.0);
        }
        if is_key_pressed(KeyCode::X) {
            camera.driver_mut::<YawPitch>().rotate_yaw_pitch(90.0, 0.0);
        }

        // Update the camera rig, and get the interpolated transform
        let camera_xform = camera.update(get_frame_time());

        clear_background(LIGHTGRAY);

        // Pass the camera to macroquad, doing some gymnastics to convince
        // the two different `glam` versions to talk to each other.
        set_camera(&Camera3D {
            position: <[f32; 3]>::from(camera_xform.position).into(),
            up: <[f32; 3]>::from(camera_xform.up()).into(),
            target: <[f32; 3]>::from(camera_xform.position + camera_xform.forward()).into(),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1.01, -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1.01, 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1.01, 2.), vec3(2., 2., 2.), YELLOW);

        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), None, WHITE);
        draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), None, WHITE);
        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        set_default_camera();
        draw_text("Press X or Z to rotate the camera", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
