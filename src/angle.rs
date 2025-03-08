use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Angle<N> {
    Radians(N),
    Degrees(N),
}

impl<N: Default> Default for Angle<N> {
    fn default() -> Self {
        Self::Degrees(N::default())
    }
}

impl<N: BaseFloat> Angle<N> {
    pub fn as_radians(&self) -> N {
        match self {
            Angle::Radians(v) => *v,
            Angle::Degrees(v) => v.to_radians(),
        }
    }

    pub fn as_degrees(&self) -> N {
        match self {
            Angle::Radians(v) => v.to_degrees(),
            Angle::Degrees(v) => *v,
        }
    }
}
