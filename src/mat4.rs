use crate::types::BaseNumber;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat4<N>
{
    contents : [N; 16]
}

impl<N> Mat4<N>
{
    pub fn new(
        contents : [N; 16]
    ) -> Self
    {
        Self
        {
            contents
        }
    }
}

impl<N : Copy> Mat4<N>
{
    pub fn transpose(&self) -> Self
    {
        Self::new([
            self.contents[0], self.contents[4], self.contents[8], self.contents[12],
            self.contents[1], self.contents[5], self.contents[9], self.contents[13],
            self.contents[2], self.contents[6], self.contents[10], self.contents[14],
            self.contents[3], self.contents[7], self.contents[11], self.contents[15],
        ])
    }
}

impl<N : BaseNumber> Mat4<N>
{
    pub fn identity() -> Mat4<N>
    {
        Self { contents: [
            N::one(), N::zero(), N::zero(), N::zero(),
            N::zero(), N::one(), N::zero(), N::zero(),
            N::zero(), N::zero(), N::one(), N::zero(),
            N::zero(), N::zero(), N::zero(), N::one(),
        ] }
    }
}

#[cfg(feature="bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature="bytemuck")]
unsafe impl<N : Pod + Zeroable> Pod for Mat4<N> {}
#[cfg(feature="bytemuck")]
unsafe impl<N : Pod + Zeroable> Zeroable for Mat4<N> {}