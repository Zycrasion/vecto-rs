use std::rc::Rc;

use crate::{Vec2, bounding_box::AABB};

pub type QuadValue<T> = Box<(Vec2, T)>;

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
    pub fn new<N: Into<f32>>(x : N, y : N, w : N, h : N, max_values: usize) -> Self
    {
        assert!(max_values > 0);

        QuadTree
        {
            bb : AABB { x:x.into(), y:y.into(), w:w.into(), h:h.into() },
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

    pub fn prune(&mut self)
    {
        if self.tl.as_ref().unwrap().len() + self.tr.as_ref().unwrap().len() + self.br.as_ref().unwrap().len() + self.bl.as_ref().unwrap().len() < self.max_values
        {
            self.join();
        } else 
        {
            self.tl.as_mut().unwrap().prune();
            self.tr.as_mut().unwrap().prune();
            self.bl.as_mut().unwrap().prune();
            self.br.as_mut().unwrap().prune();
        }
    }

    fn join(&mut self)
    {
        if self.is_leaf() {return;}
        self.values.clear();
        self.values.append(&mut self.tl.as_mut().unwrap().remove_values());
        self.values.append(&mut self.tr.as_mut().unwrap().remove_values());
        self.values.append(&mut self.bl.as_mut().unwrap().remove_values());
        self.values.append(&mut self.br.as_mut().unwrap().remove_values());
    }

    fn point_inside(&self, p : Vec2) -> bool
    {
        self.bb.point_inside(p)
    }

    fn remove_values(&mut self) -> Vec<QuadValue<T>>
    {
        if self.is_leaf()
        {
            return self.values.drain(0..self.values.len()).collect();
        }

        let mut values : Vec<QuadValue<T>> = Vec::new();

        values.append(&mut self.tl.as_mut().unwrap().remove_values());
        values.append(&mut self.tr.as_mut().unwrap().remove_values());
        values.append(&mut self.bl.as_mut().unwrap().remove_values());
        values.append(&mut self.br.as_mut().unwrap().remove_values());

        values
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
            self.values.push(Box::new((p, v)));

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

    pub fn len(&self) -> usize
    {
        self.values.len()
    }

    pub fn remove(&mut self, p : Vec2) -> Option<QuadValue<T>>
    {
        if self.is_leaf()
        {
            let mut index = None;
            let mut i = 0;
            for v in &self.values
            {
                if p == v.as_ref().0
                {
                    index = Some(i);
                }

                i += 1;
            }

            if let Some(i) = index
            {
                return Some(self.values.remove(i));
            }
        } else 
        {
            if self.tl.as_mut().unwrap().point_inside(p)
            {
                return self.tl.as_mut().unwrap().remove(p);
            }
            if self.tr.as_mut().unwrap().point_inside(p)
            {
                return self.tr.as_mut().unwrap().remove(p);
            }
            if self.bl.as_mut().unwrap().point_inside(p)
            {
                return self.bl.as_mut().unwrap().remove(p);
            }
            if self.br.as_mut().unwrap().point_inside(p)
            {
                return self.br.as_mut().unwrap().remove(p);
            }
        }
        None
    }

    fn is_leaf(&self) -> bool
    {
        return self.tr.is_none()
    }
}