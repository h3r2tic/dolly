//! Composable cameras for simple games and apps.
//!
//! A `dolly` [`CameraRig`] comprises a chain of [`RigDriver`]s. Each driver in the chain applies
//! a transformation or an animation on top of the previous one.
//!
//! Every driver provides runtime controls over its functionality, e.g. [`YawPitch`] calculates
//! rotation via yaw and pitch angles. Each frame, driver parameters can be modified,
//! and will affect the subsequent call to [`CameraRig::update`], which provides the final camera transformation.
//!
//! Please note that `dolly` currently assumes the right-handed OpenGL-style coordinate system.
//!
//! # Example
//!
//! ```
//! use dolly::prelude::*;

//! let mut camera = CameraRig::builder()
//!     .with(Position::new(glam::Vec3::ZERO))
//!     .with(YawPitch::new())
//!     .with(Smooth::new_position_rotation(1.0, 1.0))
//!     .build();
//!
//! // ...
//!
//! camera
//!     .driver_mut::<YawPitch>()
//!     .rotate_yaw_pitch(10.0, 0.0);
//! camera.update(1.0 / 60.0);
//! ```
//!
//! [`CameraRig`]: rig/struct.CameraRig.html
//! [`RigDriver`]: driver/trait.RigDriver.html
//! [`YawPitch`]: drivers/yaw_pitch/struct.YawPitch.html
//! [`CameraRig::update`]: rig/struct.CameraRig.html#method.update

pub mod driver;
pub mod drivers;
pub mod prelude;
pub mod rig;
pub mod transform;

mod util;

pub use glam;
