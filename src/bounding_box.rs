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
}