mod bounding_box;

mod quadtree;

mod triangles;

mod line;

#[macro_use]
mod private_macros;

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