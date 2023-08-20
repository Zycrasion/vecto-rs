use crate::Vec2;

use super::leaf::Leaf;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AABB
{
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
}

pub trait QuadtreeChild<T>
{
    fn get_from_position(&self, p : Vec2) -> Option<T>;
    fn add(&mut self, v : T, p : Vec2, max_items_per_leaf : u32) -> Result<(), ()>;
}

pub struct Quadtree<T>
{
    items_per_leaf : u32,
    bounding_box : AABB,
    TL : Box<dyn QuadtreeChild<T>>,
    TR : Box<dyn QuadtreeChild<T>>,
    BL : Box<dyn QuadtreeChild<T>>,
    BR : Box<dyn QuadtreeChild<T>>,    
}

impl<T: 'static> Quadtree<T>
{
    pub fn new(items_per_leaf : u32, bounding_box : AABB) -> Self
    {
        assert_ne!(items_per_leaf, 0);

        Self
        {
            items_per_leaf,
            bounding_box,

            TL : Box::new(Leaf::new(bounding_box, 0, 0)),
            TR : Box::new(Leaf::new(bounding_box, 1, 0)),
            BL : Box::new(Leaf::new(bounding_box, 0, 1)),
            BR : Box::new(Leaf::new(bounding_box, 1, 1))
        }
    }

    pub fn insert(&mut self, v : T, p : Vec2)
    {

    }
}