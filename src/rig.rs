use crate::{driver::RigDriver, transform::Transform};

/// A chain of drivers, calculating displacements, and animating in succession.
pub struct CameraRig {
    ///
    pub drivers: Vec<Box<dyn RigDriver>>,

    ///
    pub final_transform: Transform,
}

// Prevents user calls to `RigDriver::update`. All updates must come from `CameraRig::update`.
struct RigUpdateToken;

///
pub struct RigUpdateParams<'a> {
    ///
    pub parent: &'a Transform,
    ///
    pub delta_time_seconds: f32,
    _token: RigUpdateToken,
}

impl CameraRig {
    /// Returns the first driver of the matching type. Panics if no such driver is present.
    pub fn driver_mut<T: RigDriver>(&mut self) -> &mut T {
        self.try_driver_mut::<T>().unwrap_or_else(|| {
            panic!(
                "No {} driver found in the CameraRig",
                std::any::type_name::<T>()
            )
        })
    }

    /// Returns the Some with the first driver of the matching type, or `None` if no such driver is present.
    pub fn try_driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        self.drivers
            .iter_mut()
            .find_map(|driver| driver.as_mut().as_any_mut().downcast_mut::<T>())
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    ///
    /// Camera rigs are approximately framerate independent, so `update` can be called at any frequency.
    pub fn update(&mut self, delta_time_seconds: f32) -> Transform {
        let mut parent_transform = Transform::IDENTITY;

        for driver in self.drivers.iter_mut() {
            let transform = driver.update(RigUpdateParams {
                parent: &parent_transform,
                delta_time_seconds,
                _token: RigUpdateToken,
            });

            parent_transform = transform;
        }

        self.final_transform = parent_transform;
        self.final_transform
    }

    /// Use this to make a new rig
    pub fn builder() -> CameraRigBuilder {
        CameraRigBuilder {
            drivers: Default::default(),
        }
    }
}

///
pub struct CameraRigBuilder {
    drivers: Vec<Box<dyn RigDriver>>,
}

impl CameraRigBuilder {
    ///
    pub fn with(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    ///
    pub fn build(self) -> CameraRig {
        let mut rig = CameraRig {
            drivers: self.drivers,
            // Initialize with a dummy identity transform. Will be overridden in a moment.
            final_transform: Transform::IDENTITY,
        };

        // Update once to find the final transform
        rig.update(0.0);
        rig
    }
}
