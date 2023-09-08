#[warn(missing_docs)]

mod bounding_box;
pub use bounding_box::AABB;

mod quadtree;
pub use quadtree::QuadTree;

mod triangles;

mod line;

mod vec;
pub use vec::*;

pub mod maths   
{
    pub use crate::line::*;
    pub use crate::triangles::*;
}