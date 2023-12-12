#[warn(missing_docs)]
mod bounding_box;

#[warn(missing_docs)]
mod quadtree;

#[warn(missing_docs)]
mod triangles;

#[warn(missing_docs)]
mod line;

#[macro_use]
pub(self) mod private_macros;

#[warn(missing_docs)]
mod vec;

#[warn(missing_docs)]
mod mat4;

#[warn(missing_docs)]
mod mat3;

#[warn(missing_docs)]
mod radians;

#[warn(missing_docs)]
/// Trigonometry related functions
pub mod trig   
{
    pub use crate::line::*;
    pub use crate::triangles::*;
    pub use crate::radians::*;
}

#[warn(missing_docs)]
/// Linear Algebra
pub mod linear
{
    pub use crate::vec::*;
    pub use crate::mat3::*;
    pub use crate::mat4::*;
}

#[warn(missing_docs)]
/// Spatial related functions
pub mod spatial
{
    pub use crate::bounding_box::AABB;
    pub use crate::quadtree::QuadTree;
}