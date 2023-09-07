use crate::Vector;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AABB
{
    pub start : Vector,
    pub size : Vector
}

impl AABB
{
    pub fn new(start : Vector, size : Vector) -> Self
    {
        AABB
        {
            start,
            size
        }
    }

    pub fn point_inside(&self, p : Vector) -> bool
    {
        return p >= self.start && p <= self.start + self.size;
    }

    pub fn intersection(&self, o : AABB) -> bool
    {
        return o.point_inside(self.start) || self.point_inside(o.start)
         ||  o.point_inside(self.start + self.size) || self.point_inside(o.start + o.size)
    }
}