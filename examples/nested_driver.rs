// Based on https://github.com/not-fl3/macroquad/blob/97a99d00155cb7531f4432a2eb5f3c587e22f9b3/examples/3d.rs

use dolly::{driver::RigDriver, prelude::*};
use macroquad::{
    prelude::{
        draw_cube, draw_cube_wires, draw_grid, draw_sphere, info, is_key_down, is_key_pressed,
        set_camera, set_default_camera, vec3, Camera3D, KeyCode, Vec3, BLACK, BLUE, DARKBLUE,
        DARKGREEN, GRAY, LIGHTGRAY, WHITE, YELLOW,
    },
    time::get_frame_time,
    window::{clear_background, next_frame},
};

/// A custom camera rig which combines smoothed movement with a look-at driver.
#[derive(Debug)]
pub struct MovableLookAt<H: Handedness>(CameraRig<H>);

// Turn the nested rig into a driver, so it can be used in another rig.
impl<H: Handedness> RigDriver<H> for MovableLookAt<H> {
    fn update(&mut self, params: dolly::rig::RigUpdateParams<H>) -> dolly::transform::Transform<H> {
        self.0.update(params.delta_time_seconds)
    }
}

impl<H: Handedness> MovableLookAt<H> {
    pub fn from_position_target(camera_position: glam::Vec3, target_position: glam::Vec3) -> Self {
        Self(
            CameraRig::builder()
                // Allow moving the camera
                .with(Position::new(camera_position))
                // Predict camera movement to make the subsequent smoothing reactive
                .with(Smooth::new_position(1.25).predictive(true))
                // Smooth the predicted movement
                .with(Smooth::new_position(2.5))
                .with(LookAt::new(target_position + glam::Vec3::Y).tracking_smoothness(1.25))
                .build(),
        )
    }

    pub fn set_position_target(
        &mut self,
        camera_position: glam::Vec3,
        target_position: glam::Vec3,
    ) {
        self.0.driver_mut::<Position>().position = camera_position.into();
        self.0.driver_mut::<LookAt>().target = target_position.into();
    }
}

#[macroquad::main("dolly nested_driver example")]
async fn main() {
    info!("{}", "WASD to move");
    info!("{}", "Spacebar and LShift to go up and down");
    info!("{}", "C to switch between player and camera");

    let mut camera_position = glam::Vec3::new(4., 3., 8.);
    let mut player_position = glam::Vec3::new(2., 1.01, 2.);

    // Create a camera rig with our custom nested `MovableLookAt` driver within.
    let mut camera = CameraRig::builder()
        .with(MovableLookAt::from_position_target(
            camera_position,
            player_position,
        ))
        .build();

    let mut is_player = true;

    loop {
        // Switch between controlling the player and controlling the camera
        if is_key_pressed(KeyCode::C) {
            is_player = !is_player;
            println!(
                "Now controlling the {}.",
                if is_player { "Player" } else { "Camera" }
            );
        }

        // Move either the player or the camera
        let delta_pos = get_move_input();
        if is_player {
            player_position += delta_pos;
        } else {
            camera_position += delta_pos;
        }

        // Update the camera driver
        camera
            .driver_mut::<MovableLookAt<RightHanded>>()
            .set_position_target(camera_position, player_position);

        // Update the camera rig, and get the interpolated transform
        let camera_xform = camera.update(get_frame_time());

        clear_background(LIGHTGRAY);

        // Pass the camera to macroquad, doing some gymnastics to convince
        // the two different `glam` versions to talk to each other.
        set_camera(&Camera3D {
            position: camera_xform.position.d2m(),
            up: camera_xform.up::<glam::Vec3>().d2m(),
            target: (glam::Vec3::from(camera_xform.position)
                + camera_xform.forward::<glam::Vec3>())
            .d2m(),
            ..Default::default()
        });

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
        draw_cube_wires(player_position.d2m(), vec3(2., 2., 2.), YELLOW);

        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), None, WHITE);
        draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), None, WHITE);
        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        set_default_camera();

        next_frame().await
    }
}

trait DollyToMacroquad {
    type Target;
    fn d2m(self) -> Self::Target;
}

impl DollyToMacroquad for glam::Vec3 {
    type Target = Vec3;

    fn d2m(self) -> Self::Target {
        <[f32; 3]>::from(self).into()
    }
}

impl DollyToMacroquad for mint::Vector3<f32> {
    type Target = Vec3;

    fn d2m(self) -> Self::Target {
        <[f32; 3]>::from(self).into()
    }
}

impl DollyToMacroquad for mint::Point3<f32> {
    type Target = Vec3;

    fn d2m(self) -> Self::Target {
        <[f32; 3]>::from(self).into()
    }
}

fn get_move_input() -> glam::Vec3 {
    const SPEED: f32 = 0.05;

    let mut delta_pos = glam::Vec3::ZERO;

    if is_key_down(KeyCode::D) {
        delta_pos.x += SPEED;
    }
    if is_key_down(KeyCode::A) {
        delta_pos.x -= SPEED;
    }
    if is_key_down(KeyCode::S) {
        delta_pos.z += SPEED;
    }
    if is_key_down(KeyCode::W) {
        delta_pos.z -= SPEED;
    }
    if is_key_down(KeyCode::LeftShift) {
        delta_pos.y -= SPEED;
    }
    if is_key_down(KeyCode::Space) {
        delta_pos.y += SPEED;
    }

    delta_pos
}
