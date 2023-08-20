use super::quadtree::QuadtreeChild;

pub struct Branch<T>
{
    contents : [Box<dyn QuadtreeChild<T>>; 4]
}

impl<T> QuadtreeChild<T> for Branch<T>
{
    fn get_from_position(&self, p : crate::Vec2) -> Option<T> {
        todo!()
    }

    fn add(&mut self, v : T, p : crate::Vec2, max_items_per_leaf : u32) -> Result<(), ()> {
        todo!()
    }
}