#[warn(missing_docs)]


mod bounding_box;

mod quadtree;

mod triangles;

mod line;

mod vec;

/// Trigonometry related functions
pub mod trig   
{
    pub use crate::line::*;
    pub use crate::triangles::*;
}

/// Positional related functions
pub mod positional
{
    pub use crate::vec::Vector;
}

/// Spatial related functions
pub mod spatial
{
    pub use crate::bounding_box::AABB;
    pub use crate::quadtree::QuadTree;
}