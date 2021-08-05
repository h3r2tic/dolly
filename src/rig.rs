use crate::{driver::RigDriver, transform::Transform};

pub struct CameraRig {
    pub drivers: Vec<Box<dyn RigDriver>>,
    pub transform: Transform,
}

struct RigUpdateToken;

pub struct RigUpdateParams<'a> {
    pub parent: &'a Transform,
    pub dt: f32,
    _token: RigUpdateToken,
}

impl CameraRig {
    /// Returns the first driver of the matching type
    pub fn driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        self.drivers
            .iter_mut()
            .find_map(|driver| Some(driver.as_mut().as_any_mut().downcast_mut::<T>()?))
    }
}

impl CameraRig {
    pub fn update(&mut self, dt: f32) -> Transform {
        let mut parent_transform = Transform::IDENTITY;

        for driver in self.drivers.iter_mut() {
            let transform = driver.update(RigUpdateParams {
                parent: &parent_transform,
                dt,
                _token: RigUpdateToken,
            });

            parent_transform = transform;
        }

        self.transform = parent_transform;
        self.transform
    }
}

pub struct CameraRigBuilder {
    drivers: Vec<Box<dyn RigDriver>>,
}

impl CameraRig {
    pub fn builder() -> CameraRigBuilder {
        CameraRigBuilder {
            drivers: Default::default(),
        }
    }
}

impl CameraRigBuilder {
    pub fn with(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    pub fn build(self) -> CameraRig {
        let mut rig = CameraRig {
            drivers: self.drivers,
            transform: Transform::IDENTITY,
        };

        rig.update(0.0);
        rig
    }
}
