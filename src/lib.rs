/// The float precision that the library was compiled with
#[cfg(not(feature = "f64"))]
pub type Float = f32;

/// The float precision that the library was compiled with
#[cfg(feature = "f64")]
pub type Float = f64;

/// PI using the precision that the library was compiled with
#[cfg(not(feature = "f64"))]
pub const PI : Float = std::f32::consts::PI;

/// PI using the precision that the library was compiled with
#[cfg(feature = "f64")]
pub const PI : Float = std::f64::consts::PI;

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
}

#[warn(missing_docs)]
/// Spatial related functions
pub mod spatial
{
    pub use crate::bounding_box::AABB;
    pub use crate::quadtree::QuadTree;
}