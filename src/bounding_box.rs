use crate::Vector;

/// An Axis Aligned Bounding Box
/// 
/// Usage is like follows
/// 
/// ```
/// use vecto_rs::*;
/// let axis_box = AABB::new(Vector::new2(0.0,0.0), Vector::new2(1.0, 1.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq)]
pub struct AABB
{
    /// The Top Left Corner
    pub start : Vector,

    /// Size of Axis Aligned Bounding Box
    pub size : Vector
}

impl AABB
{
    /// Create a new AABB by giving it the Top Left Corner and Size of Box
    /// ```
    /// pub use vecto_rs::*;
    /// AABB::new(Vector::new2(0.0,0.0), Vector::new2(1.0, 1.0));
    /// ```
    pub fn new(start : Vector, size : Vector) -> Self
    {
        AABB
        {
            start,
            size
        }
    }

    /// Check if a Vector is inside the Box
    pub fn point_inside(&self, p : Vector) -> bool
    {
        return p >= self.start && p <= self.start + self.size;
    }

    /// Check if another AABB is inside this one
    pub fn intersection(&self, o : AABB) -> bool
    {
        return  (self.start.x <= o.start.x + o.size.x && self.start.x + self.size.x >= o.start.x) &&
                (self.start.y <= o.start.y + o.size.y && self.start.y + self.size.y >= o.start.y) &&
                (self.start.z <= o.start.z + o.size.z && self.start.z + self.size.z >= o.start.z)
    }
}