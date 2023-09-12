#[warn(missing_docs)]
mod bounding_box;

#[warn(missing_docs)]
mod quadtree;

#[warn(missing_docs)]
mod triangles;

#[warn(missing_docs)]
mod line;

#[macro_use]
pub(crate) mod private_macros;

mod vec;

#[warn(missing_docs)]
/// Trigonometry related functions
pub mod trig   
{
    pub use crate::line::*;
    pub use crate::triangles::*;
}

#[warn(missing_docs)]
/// Positional related functions
pub mod positional
{
    pub use crate::vec::Vector;
}

#[warn(missing_docs)]
/// Spatial related functions
pub mod spatial
{
    pub use crate::bounding_box::AABB;
    pub use crate::quadtree::QuadTree;
}