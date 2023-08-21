use crate::Vec2;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AABB
{
    pub start : Vec2,
    pub size : Vec2
}

impl AABB
{
    pub fn new(start : Vec2, size : Vec2) -> Self
    {
        AABB
        {
            start,
            size
        }
    }

    pub fn point_inside(&self, p : Vec2) -> bool
    {
        // I love my vector class
        return p >= self.start && p <= self.start + self.size;
    }

    pub fn intersection(&self, o : AABB) -> bool
    {
        return o.point_inside(self.start) || self.point_inside(o.start)
         ||  o.point_inside(self.start + self.size) || self.point_inside(o.start + o.size)
    }
}