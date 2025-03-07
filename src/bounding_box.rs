use crate::prelude::*;

/// An Axis Aligned Bounding Box
/// 
/// Usage is like follows
/// 
/// ```
/// use vecto_rs::spatial::AABB;
/// use vecto_rs::linear::*;
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
    /// use vecto_rs::spatial::AABB;
    /// use vecto_rs::linear::*;
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
    /// ```
    /// use vecto_rs::spatial::AABB;
    /// use vecto_rs::linear::*;
    /// let a = AABB::new(Vector::new2(0.0, 0.0),Vector::new2(5.0, 5.0));
    /// assert!(a.point_inside(Vector::new2(1.0,1.0)));
    /// ```
    pub fn point_inside(&self, p : Vector) -> bool
    {
        return  (p.x >= self.start.x && p.x <= self.start.x + self.size.x) && 
                (p.y >= self.start.y && p.y <= self.start.y + self.size.y) && 
                (p.z >= self.start.z && p.z <= self.start.z + self.size.z) 
    }

    /// Check if another AABB is inside this one
    /// ```
    /// use vecto_rs::spatial::AABB;
    /// use vecto_rs::linear::*;
    /// let a = AABB::new(Vector::new2(0.0, 0.0),Vector::new2(5.0, 5.0));
    /// let b = AABB::new(Vector::new2(2.5, 0.0),Vector::new2(5.0, 5.0));
    /// assert!(a.intersection(b));
    /// ```
    pub fn intersection(&self, o : AABB) -> bool
    {
        return  (self.start.x <= o.start.x + o.size.x && self.start.x + self.size.x >= o.start.x) &&
                (self.start.y <= o.start.y + o.size.y && self.start.y + self.size.y >= o.start.y) &&
                (self.start.z <= o.start.z + o.size.z && self.start.z + self.size.z >= o.start.z)
    }
}