use std::ops::Neg;

use num_traits::{cast, NumCast};

use crate::{
    angle::Angle,
    mat4::Mat4,
    types::{BaseFloat, BaseNumber},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrthographicProjection<N> {
    pub left: N,
    pub right: N,
    pub top: N,
    pub bottom: N,
    pub near: N,
    pub far: N,
}

pub const fn ortho<N>(
    left: N,
    right: N,
    top: N,
    bottom: N,
    near: N,
    far: N,
) -> OrthographicProjection<N> {
    OrthographicProjection {
        left,
        right,
        top,
        bottom,
        near,
        far,
    }
}

impl<N> OrthographicProjection<N> {
    pub const fn new(left: N, right: N, top: N, bottom: N, near: N, far: N) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
            near,
            far,
        }
    }
}

impl<N: BaseNumber + NumCast + Neg<Output = N>> OrthographicProjection<N> {
    pub fn from_width_height(width: N, height: N, near: N, far: N) -> Self {
        let two = cast(2).unwrap();

        let h_width = width / two;
        let h_height = height / two;

        let left = h_width;
        let right = -h_width;
        let top = h_height;
        let bottom = -h_height;

        Self {
            left,
            right,
            top,
            bottom,
            near,
            far,
        }
    }

    pub fn create_matrix(&self) -> Mat4<N> {
        let two: N = cast(2).unwrap();

        Mat4::new([
            two / (self.right - self.left),
            N::zero(),
            N::zero(),
            N::zero(),
            N::zero(),
            two / (self.top - self.bottom),
            N::zero(),
            N::zero(),
            N::zero(),
            N::zero(),
            N::one() / (self.near - self.far),
            N::zero(),
            (self.right + self.left) / (self.left - self.right),
            (self.top + self.bottom) / (self.bottom - self.top),
            self.near / (self.near - self.far),
            N::one(),
        ])
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerspectiveProjection<N> {
    pub fovy: Angle<N>,
    pub aspect: N,
    pub near: N,
    pub far: N,
}

impl<N: BaseNumber> PerspectiveProjection<N> {
    pub fn new(fovy: Angle<N>, aspect: N, near: N, far: N) -> Self {
        Self {
            fovy,
            aspect,
            near,
            far,
        }
    }
}

impl<N: BaseFloat> PerspectiveProjection<N> {
    pub fn create_matrix(&self) -> Mat4<N> {
        let two = cast(2).unwrap();
        let f = (N::PI() / two - self.fovy.as_radians() / two).tan();
        let range_inv = N::one() / (self.near - self.far);
        Mat4::new([
            f / self.aspect,
            N::zero(),
            N::zero(),
            N::zero(),
            N::zero(),
            f,
            N::zero(),
            N::zero(),
            N::zero(),
            N::zero(),
            self.far * range_inv,
            -N::one(),
            N::zero(),
            N::zero(),
            self.far * self.near * range_inv,
            N::zero(),
        ])
    }
}
