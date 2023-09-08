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
        return  (self.start.x <= o.start.x + o.size.x && self.start.x + self.size.x >= o.start.x) &&
                (self.start.y <= o.start.y + o.size.y && self.start.y + self.size.y >= o.start.y) &&
                (self.start.z <= o.start.z + o.size.z && self.start.z + self.size.z >= o.start.z)
    }
}