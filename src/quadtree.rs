use std::rc::Rc;

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

impl<T: Clone> QuadTree<T>
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

    fn subdivide(&mut self)
    {
        self.tl = Some(Box::new(QuadTree::new(self.bb.x                  , self.bb.y                  , self.bb.w / 2.0, self.bb.h / 2.0, self.max_values)));
        self.tr = Some(Box::new(QuadTree::new(self.bb.x + self.bb.w / 2.0, self.bb.y                  , self.bb.w / 2.0, self.bb.h / 2.0, self.max_values)));
        self.bl = Some(Box::new(QuadTree::new(self.bb.x                  , self.bb.y + self.bb.h / 2.0, self.bb.w / 2.0, self.bb.h / 2.0, self.max_values)));
        self.br = Some(Box::new(QuadTree::new(self.bb.x + self.bb.w / 2.0, self.bb.y + self.bb.h / 2.0, self.bb.w / 2.0, self.bb.h / 2.0, self.max_values)));
        
        let values : Vec<_> = self.values.drain(0..self.values.len()).collect();
        for v in &values
        {
            let v1 = v.1.clone();
            let p = v.0;
            self.add(v1, p);
        }
    
    }

    fn point_inside(&self, p : Vec2) -> bool
    {
        self.bb.point_inside(p)
    }

    pub fn query(&mut self, p : Vec2) -> &mut Vec<QuadValue<T>>
    {
        if self.is_leaf()
        {
            return &mut self.values
        } else 
        {
            if self.tl.as_mut().unwrap().point_inside(p)
            {
                return self.tl.as_mut().unwrap().query(p);
            }
            if self.tr.as_mut().unwrap().point_inside(p)
            {
                return self.tr.as_mut().unwrap().query(p);
            }
            if self.bl.as_mut().unwrap().point_inside(p)
            {
                return self.bl.as_mut().unwrap().query(p);
            }
            if self.br.as_mut().unwrap().point_inside(p)
            {
                return self.br.as_mut().unwrap().query(p);
            }
        }
        &mut self.values
    }

    pub fn add(&mut self, v : T, p : Vec2)
    {
        if self.is_leaf()
        {
            self.values.push((p, v));

            if self.values.len() > self.max_values
            {
                self.subdivide()
            }

        } else 
        {
            if self.tl.as_mut().unwrap().point_inside(p)
            {
                return self.tl.as_mut().unwrap().add(v, p);
            }
            if self.tr.as_mut().unwrap().point_inside(p)
            {
                return self.tr.as_mut().unwrap().add(v, p);
            }
            if self.bl.as_mut().unwrap().point_inside(p)
            {
                return self.bl.as_mut().unwrap().add(v, p);
            }
            if self.br.as_mut().unwrap().point_inside(p)
            {
                return self.br.as_mut().unwrap().add(v, p);
            }
        }
    }

    fn is_leaf(&self) -> bool
    {
        return self.tr.is_some()
    }
}