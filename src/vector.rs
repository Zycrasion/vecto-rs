use std::ops::*;

use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

#[cfg(feature="bytemuck")]
pub use bytemuck::{Pod, Zeroable};

#[cfg(feature="bytemuck")]
unsafe impl<N : Pod + Zeroable> Zeroable for Vector3<N> {}

#[cfg(feature="bytemuck")]
unsafe impl<N : Pod + Zeroable> Pod for Vector3<N> {}

pub const fn vec3<N>(x: N, y: N, z: N) -> Vector3<N> {
    Vector3::new(x, y, z)
}

impl<N> Vector3<N> {
    pub const fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N: Copy> Vector3<N> {
    pub const fn from_one(xyz: N) -> Self {
        Self::new(xyz, xyz, xyz)
    }

    pub const fn from_tuple(tuple: &(N, N, N)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }

    pub const fn to_tuple(&self) -> (N, N, N) {
        (self.x, self.y, self.z)
    }
}

impl Vector3<f32>
{
    pub const X : Vector3<f32> = Vector3::new(1., 0., 0.);
    pub const Y : Vector3<f32> = Vector3::new(0., 1., 0.);
    pub const Z : Vector3<f32> = Vector3::new(0., 0., 1.);
    pub const ZERO : Vector3<f32> = Vector3::from_one(0.);
    pub const ONE : Vector3<f32> = Vector3::from_one(1.);
}

impl Vector3<f64>
{
    pub const X : Vector3<f64> = Vector3::new(1., 0., 0.);
    pub const Y : Vector3<f64> = Vector3::new(0., 1., 0.);
    pub const Z : Vector3<f64> = Vector3::new(0., 0., 1.);
    pub const ZERO : Vector3<f64> = Vector3::from_one(0.);
    pub const ONE : Vector3<f64> = Vector3::from_one(1.);
}


impl<N: BaseNumber> Vector3<N> {
    pub fn magnitude_sq(&self) -> N {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dist_sq(&self, other: &Self) -> N {
        (self - other).magnitude_sq()
    }
}

impl<N: BaseFloat> Vector3<N> {
    pub fn magnitude(&self) -> N {
        self.magnitude_sq().sqrt()
    }

    pub fn dist(&self, other: &Self) -> N {
        self.dist_sq(other).sqrt()
    }

    pub fn lerp(&self, other: &Self, t: N) -> Self {
        Self::new(
            t * (self.x + other.x),
            t * (self.y + other.y),
            t * (self.z + other.z),
        )
    }

    pub fn normalized(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, other: &Self) -> N {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn angle_between(&self, other: &Self) -> Angle<N> {
        let cross = self.cross(other);

        Angle::Radians(N::atan2(cross.magnitude(), self.dot(other)) * cross.z.signum())
    }
}

macro_rules! impl_op {
    ($Op:tt $func:tt $op:tt $ty:ty) => {
        impl<N: BaseNumber> $Op<N> for $ty {
            type Output = Vector3<N>;
            fn $func(self, rhs: N) -> Self::Output {
                vec3(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }
        impl<N: BaseNumber> $Op<$ty> for $ty {
            type Output = Vector3<N>;
            fn $func(self, rhs: $ty) -> Self::Output {
                vec3(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }
    };
    ($Op:tt $func:tt $op:tt) => {
        impl_op!($Op $func $op Vector3<N>);
        impl_op!($Op $func $op &Vector3<N>);
    };
}

impl_op!(Div div /);
impl_op!(Mul mul *);
impl_op!(Add add +);
impl_op!(Sub sub -);

#[cfg(test)]
mod vector3 {
    use super::Vector3;

    const VECTOR3: Vector3<f32> = Vector3::from_tuple(&(0., 1., 2.));
    const ZERO: Vector3<f32> = Vector3::from_tuple(&(0., 0., 0.));

    #[test]
    pub fn test_tuples() {
        assert_eq!(VECTOR3.to_tuple(), (0., 1., 2.));
    }

    #[test]
    pub fn test_dist_sq() {
        assert_eq!(VECTOR3.dist_sq(&ZERO), 5.);
        assert_eq!(VECTOR3.dist(&ZERO), 5_f32.sqrt());
    }
}
