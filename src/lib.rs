mod vec2;
pub use vec2::Vec2;

mod vec3;
pub use vec3::Vec3;

mod bounding_box;
pub use bounding_box::AABB;

mod quadtree;
pub use quadtree::QuadTree;

mod triangles;

mod line;

pub mod maths   
{
    pub use crate::line::*;
    pub use crate::triangles::*;
}