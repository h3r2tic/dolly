// Based on https://github.com/not-fl3/macroquad/blob/97a99d00155cb7531f4432a2eb5f3c587e22f9b3/examples/3d.rs

use dolly::prelude::*;
use macroquad::{
    prelude::{
        draw_cube, draw_cube_wires, draw_grid, draw_sphere, is_key_pressed, set_camera,
        set_default_camera, vec3, Camera3D, KeyCode, BLACK, BLUE, DARKBLUE, DARKGREEN, GRAY,
        LIGHTGRAY, WHITE, YELLOW,
    },
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, next_frame},
};

#[macroquad::main("dolly orbit example")]
async fn main() {
    // Create a smoothed orbit camera
    let mut camera: CameraRig = CameraRig::builder()
        .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
        .with(Smooth::new_rotation(1.5))
        .with(Arm::new(glam::Vec3::Z * 8.0))
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
            up: <[f32; 3]>::from(camera_xform.up::<glam::Vec3>()).into(),
            target: <[f32; 3]>::from(
                glam::Vec3::from(camera_xform.position) + camera_xform.forward::<glam::Vec3>(),
            )
            .into(),
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
