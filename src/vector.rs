use std::ops::{Div, DivAssign};

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

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

impl<N: BaseNumber> Vector3<N> {
    pub fn magnitude_sq(&self) -> N {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<N: BaseFloat> Vector3<N> {
    pub fn magnitude(&self) -> N {
        self.magnitude_sq().sqrt()
    }

    pub fn normalized(&self) -> Self {
        self / self.magnitude()
    }
}

macro_rules! impl_scalar {
    ($Op:ident<$Type:tt>) => {
        impl<N: BaseNumber> Div<N> for &$Type<N> {
            type Output = $Type<N>;

            fn div(self, rhs: N) -> Self::Output {
                vec3(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }
        impl<N: BaseNumber> Div<N> for $Type<N> {
            type Output = $Type<N>;

            fn div(self, rhs: N) -> Self::Output {
                vec3(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }
        impl<N: BaseNumber> DivAssign<N> for $Type<N> {
            fn div_assign(&mut self, rhs: N) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }
    };
}

impl_scalar!(Div<Vector3>);

#[cfg(test)]
mod vector3 {
    use super::Vector3;

    const VECTOR3: Vector3<f32> = Vector3::from_tuple(&(0., 1., 2.));

    #[test]
    pub fn test_tuples() {
        assert_eq!(VECTOR3.to_tuple(), (0., 1., 2.));
    }
}
