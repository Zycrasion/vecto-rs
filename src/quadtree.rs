use crate::{Vec2, bounding_box::AABB};

pub type QuadValue<T> = (Vec2, T);

pub struct QuadTree<T>
{
    bb : AABB,
    max_values : usize,
    values : Vec<QuadValue<T>>,
    tr : Option<Box<QuadTree<T>>>,
    tl : Option<Box<QuadTree<T>>>,
    br : Option<Box<QuadTree<T>>>,
    bl : Option<Box<QuadTree<T>>>,
}

impl<T> QuadTree<T>
{
    pub fn new(x : f32, y : f32, w : f32, h : f32, max_values: usize) -> Self
    {
        assert!(max_values > 0);

        QuadTree
        {
            bb : AABB { x, y, w, h },
            values: Vec::new(),
            tr: None,
            tl: None,
            br: None,
            bl: None,
            max_values
        }
    }
}