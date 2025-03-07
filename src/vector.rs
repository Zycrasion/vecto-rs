use crate::types::BaseNumber;

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

impl<N : BaseNumber> Vector3<N>
{
    pub const fn from_tuple(tuple : &(N, N, N)) -> Self
    {
        Self::new(tuple.0, tuple.1, tuple.2)
    }

    pub const fn to_tuple(&self) -> (N, N, N)
    {
        (self.x, self.y, self.z)
    }

    pub fn magnitude(&self) -> N
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}