use core::fmt::Debug;
use glam::{Quat, Vec3};
use std::marker::PhantomData;

use crate::handedness::Handedness;

/// A thin wrapper over a `Point3<f32>` and a `Quaternion<f32>`
#[derive(Clone, Copy, Debug)]
pub struct Transform<H: Handedness> {
    pub position: mint::Point3<f32>,
    pub rotation: mint::Quaternion<f32>,
    pub phantom: PhantomData<H>,
}

impl<H: Handedness> Transform<H> {
    pub fn from_position_rotation<P, Q>(position: P, rotation: Q) -> Self
    where
        P: Into<mint::Point3<f32>>,
        Q: Into<mint::Quaternion<f32>>,
    {
        let position = position.into();
        let rotation = rotation.into();

        Self {
            position,
            rotation,
            phantom: PhantomData,
        }
    }

    pub fn into_position_rotation<P, Q>(self) -> (P, Q)
    where
        P: From<mint::Point3<f32>>,
        Q: From<mint::Quaternion<f32>>,
    {
        (From::from(self.position), From::from(self.rotation))
    }

    /// +X
    pub fn right<V>(&self) -> V
    where
        V: From<mint::Vector3<f32>>,
    {
        let rotation: Quat = self.rotation.into();
        From::from((rotation * Vec3::X).into())
    }

    /// +Y
    pub fn up<V>(&self) -> V
    where
        V: From<mint::Vector3<f32>>,
    {
        let rotation: Quat = self.rotation.into();
        From::from((rotation * Vec3::Y).into())
    }

    /// +/-Z
    pub fn forward<V>(&self) -> V
    where
        V: From<mint::Vector3<f32>>,
    {
        let rotation: Quat = self.rotation.into();
        From::from((rotation * H::FORWARD).into())
    }

    pub const IDENTITY: Transform<H> = Transform {
        position: mint::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: mint::Quaternion {
            v: mint::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            s: 1.0,
        },
        phantom: PhantomData,
    };
}
