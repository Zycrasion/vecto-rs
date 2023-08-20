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
    pub fn point_inside(&self, p : Vec2) -> bool
    {
        return (p.0 >= self.x && p.0 <= self.x + self.w) && (p.1 >= self.y && p.1 <= self.y + self.h);
    }
}