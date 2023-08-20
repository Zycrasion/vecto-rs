use crate::Vec2;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AABB
{
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
}

impl AABB
{
    pub fn new(start : Vec2, size : Vec2) -> Self
    {
        AABB
        {
            x : start.0,
            y : start.1,
            w : size.0,
            h : size.1
        }
    }

    pub fn point_inside(&self, p : Vec2) -> bool
    {
        return (p.0 >= self.x && p.0 <= self.x + self.w) && (p.1 >= self.y && p.1 <= self.y + self.h);
    }
}