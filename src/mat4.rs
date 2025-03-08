use std::ops::{Mul, Neg};

use crate::{
    angle::Angle,
    types::{BaseFloat, BaseNumber},
    vector::Vector3,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat4<N> {
    contents: [N; 16],
}

impl<N> Mat4<N> {
    pub const fn new(contents: [N; 16]) -> Self {
        Self { contents }
    }
}

impl<N: Copy> Mat4<N> {
    pub const fn transpose(&self) -> Self {
        Self::new([
            self.contents[0],
            self.contents[4],
            self.contents[8],
            self.contents[12],
            self.contents[1],
            self.contents[5],
            self.contents[9],
            self.contents[13],
            self.contents[2],
            self.contents[6],
            self.contents[10],
            self.contents[14],
            self.contents[3],
            self.contents[7],
            self.contents[11],
            self.contents[15],
        ])
    }
}

impl<N: BaseNumber> Mat4<N> {
    pub fn identity() -> Mat4<N> {
        Self {
            contents: [
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }

    pub fn uniform_scaling(scale: N) -> Mat4<N> {
        Self {
            contents: [
                scale,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                scale,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                scale,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }

    pub fn scaling(scale: Vector3<N>) -> Mat4<N> {
        Self {
            contents: [
                scale.x,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                scale.y,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                scale.z,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }

    pub fn translation_matrix(v: Vector3<N>) -> Mat4<N> {
        Self {
            contents: [
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                v.x,
                v.y,
                v.z,
                N::one(),
            ],
        }
    }

    pub fn translate(&mut self, v: Vector3<N>) -> &mut Mat4<N> {
        *self = *self * Self::translation_matrix(v);
        self
    }
}

impl<N: BaseNumber + Neg<Output = N>> Mat4<N> {
    pub fn inverse(&self) -> Option<Mat4<N>> {
        // https://stackoverflow.com/a/1148405
        let m = self.contents;
        let mut inv = [N::zero(); 16];

        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];
        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];

        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];

        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];

        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
            - m[9] * m[3] * m[14]
            - m[13] * m[2] * m[11]
            + m[13] * m[3] * m[10];

        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
            + m[8] * m[3] * m[14]
            + m[12] * m[2] * m[11]
            - m[12] * m[3] * m[10];

        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
            - m[8] * m[3] * m[13]
            - m[12] * m[1] * m[11]
            + m[12] * m[3] * m[9];

        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
            + m[8] * m[2] * m[13]
            + m[12] * m[1] * m[10]
            - m[12] * m[2] * m[9];

        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
            + m[5] * m[3] * m[14]
            + m[13] * m[2] * m[7]
            - m[13] * m[3] * m[6];

        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
            - m[4] * m[3] * m[14]
            - m[12] * m[2] * m[7]
            + m[12] * m[3] * m[6];

        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
            + m[4] * m[3] * m[13]
            + m[12] * m[1] * m[7]
            - m[12] * m[3] * m[5];

        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
            - m[4] * m[2] * m[13]
            - m[12] * m[1] * m[6]
            + m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
            - m[5] * m[3] * m[10]
            - m[9] * m[2] * m[7]
            + m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
            + m[4] * m[3] * m[10]
            + m[8] * m[2] * m[7]
            - m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
            - m[4] * m[3] * m[9]
            - m[8] * m[1] * m[7]
            + m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
            + m[4] * m[2] * m[9]
            + m[8] * m[1] * m[6]
            - m[8] * m[2] * m[5];

        let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == N::zero() {
            return None;
        }

        let det = N::one() / det;

        for i in 0..16 {
            inv[i] *= det;
        }

        Some(Self::new(inv))
    }
}

impl<N: BaseFloat> Mat4<N> {
    pub fn rotation_x(angle: Angle<N>) -> Self {
        let radians = angle.as_radians();

        let c = radians.cos();
        let s = radians.sin();

        Self {
            contents: [
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                c,
                s,
                N::zero(),
                N::zero(),
                -s,
                c,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }

    pub fn rotation_y(angle: Angle<N>) -> Self {
        let radians = angle.as_radians();

        let c = radians.cos();
        let s = radians.sin();

        Self {
            contents: [
                c,
                N::zero(),
                -s,
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                N::zero(),
                s,
                N::zero(),
                c,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }

    pub fn rotation_z(angle: Angle<N>) -> Self {
        let radians = angle.as_radians();

        let c = radians.cos();
        let s = radians.sin();

        Self {
            contents: [
                c,
                s,
                N::zero(),
                N::zero(),
                -s,
                c,
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::zero(),
                N::one(),
            ],
        }
    }
}

impl<N: BaseNumber> Mul<Mat4<N>> for Mat4<N> {
    type Output = Mat4<N>;

    fn mul(self, rhs: Mat4<N>) -> Self::Output {
        let l = self.contents;
        let r = rhs.contents;
        let mut contents = [N::zero(); 16];

        for i in 0..4
        {
            for j in 0..4
            {
                // row * column
                contents[i + j * 4] =   l[0 + j * 4] * r[i + 0 * 4] + 
                                        l[1 + j * 4] * r[i + 1 * 4] + 
                                        l[2 + j * 4] * r[i + 2 * 4] + 
                                        l[3 + j * 4] * r[i + 3 * 4];
            }
        }

        Mat4::new(contents)
    }
}

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "bytemuck")]
unsafe impl<N: Pod + Zeroable> Pod for Mat4<N> {}
#[cfg(feature = "bytemuck")]
unsafe impl<N: Pod + Zeroable> Zeroable for Mat4<N> {}

#[cfg(test)]
mod mat4 {
    use super::Mat4;

    #[test]
    fn identity_mul() {
        let identity: Mat4<f32> = Mat4::identity();
        let abc = Mat4::new([
            0., 1., 5., 3., 10., 12., 19123., 16., -123., 11416534., 12312., 16., -123., 123.,
            4687567., 123.,
        ]);

        assert_eq!(identity * abc, abc);
        assert_eq!(abc * identity, abc);
        assert_eq!(identity * identity, identity);
    }
}
