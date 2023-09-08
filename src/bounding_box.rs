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
        return self.start <= o.start + o.size && o.start >= self.start + self.size
    }
}