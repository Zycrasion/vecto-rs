use super::quadtree::{QuadtreeChild, AABB};

#[derive(Clone)]
pub struct Leaf<T>
{
    contents : Vec<T>,
    bb : AABB
}

impl<T> QuadtreeChild<T> for Leaf<T>
{
    fn get_from_position(&self, p : crate::Vec2) -> Option<T> {
        todo!()
    }

    fn add(&mut self, v : T, p : crate::Vec2, max_items_per_leaf : u32) -> Result<(), ()> {
        todo!()
    }
}

impl<T> Leaf<T>
{
    pub fn new(super_bb : AABB, x : u32, y : u32) -> Self
    {
        let start_x = super_bb.x + ((super_bb.w / 2.0) * x as f32);
        let start_y = super_bb.y + ((super_bb.h / 2.0) * y as f32);

        Self
        {
            contents : Vec::new(),
            bb : AABB { x: start_x, y: start_y, w: super_bb.w / 2.0, h: super_bb.h / 2.0 }
        }
    }
}