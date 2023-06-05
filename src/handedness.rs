use std::fmt::Debug;

use glam::Vec3;

pub trait Handedness: Clone + Copy + Debug + 'static {
    const FORWARD_Z_SIGN: f32;
    const FORWARD: Vec3 = glam::vec3(0.0, 0.0, Self::FORWARD_Z_SIGN);

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>;
    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded;

impl Handedness for LeftHanded {
    const FORWARD_Z_SIGN: f32 = 1.0;

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>,
    {
        let up: Vec3 = up.into().into();
        let forward: Vec3 = forward.into().into();

        let result = up.cross(forward);
        From::from(result.into())
    }

    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>,
    {
        let right: Vec3 = right.into().into();
        let forward: Vec3 = forward.into().into();

        let result = forward.cross(right);
        From::from(result.into())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded;

impl Handedness for RightHanded {
    const FORWARD_Z_SIGN: f32 = -1.0;

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>,
    {
        let up: Vec3 = up.into().into();
        let forward: Vec3 = forward.into().into();

        let result = forward.cross(up);
        From::from(result.into())
    }

    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<mint::Vector3<f32>>,
        U: From<mint::Vector3<f32>>,
    {
        let right: Vec3 = right.into().into();
        let forward: Vec3 = forward.into().into();

        let result = right.cross(forward);
        From::from(result.into())
    }
}
