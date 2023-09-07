use crate::{Vector, bounding_box::AABB};

pub type QuadValue<T> = Box<(Vector, T)>;

#[derive(Clone)]
pub struct QuadTree<T : Clone>
{
    bb : AABB,
    bb_raw : AABB,
    max_values : usize,
    max_depth : u32,
    values : Vec<QuadValue<T>>,
    border_size : f32,
    tr : Option<Box<QuadTree<T>>>,
    tl : Option<Box<QuadTree<T>>>,
    br : Option<Box<QuadTree<T>>>,
    bl : Option<Box<QuadTree<T>>>,
}

impl<T: Clone + PartialEq> QuadTree<T>
{
    pub fn new(x : f32, y : f32, w : f32, h : f32, max_values: usize, border_size: f32, max_depth : u32) -> Self
    {
        assert!(max_values > 0);

        QuadTree
        {
            bb : AABB::new(Vector::new2(x - border_size / 2.0, y - border_size / 2.0), Vector::new2(w + border_size, h + border_size)),
            bb_raw : AABB { start: Vector::new2(x,y), size: Vector::new2(w, h) },
            values: Vec::new(),
            tr: None,
            tl: None,
            br: None,
            bl: None,
            max_values,
            border_size,
            max_depth
        }
    }

    fn from_bb(start : Vector, size : Vector, max_values: usize, border_size : f32, max_depth : u32) -> Self
    {
        QuadTree::new(start.x, start.y, size.x, size.y, max_values, border_size, max_depth)
    }

    fn subdivide(&mut self)
    {
        if self.max_depth == 0
        {
            return;
        }
        self.tl = Some(Box::new(QuadTree::from_bb(self.bb_raw.start, self.bb_raw.size / 2.0, self.max_values, self.border_size, self.max_depth - 1)));
        self.tr = Some(Box::new(QuadTree::from_bb(self.bb_raw.start + Vector::new2(self.bb_raw.size.x/2.0, 0.0), self.bb_raw.size / 2.0, self.max_values, self.border_size, self.max_depth - 1)));
        self.bl = Some(Box::new(QuadTree::from_bb(self.bb_raw.start + Vector::new2(0.0,self.bb_raw.size.y/2.0), self.bb_raw.size / 2.0, self.max_values, self.border_size, self.max_depth - 1)));
        self.br = Some(Box::new(QuadTree::from_bb(self.bb_raw.start + self.bb_raw.size / 2.0, self.bb_raw.size / 2.0, self.max_values, self.border_size, self.max_depth - 1)));

        let values : Vec<_> = self.values.drain(0..self.values.len()).collect();
        for v in &values
        {
            let v1 = v.1.clone();
            let p = v.0;
            self.add(v1, p);
        }
    }

    pub fn get_bb(&self) -> AABB
    {
        self.bb.clone()
    }

    pub fn get_tl(&self) -> Box<QuadTree<T>>
    {
        self.tl.clone().unwrap()
    }

    pub fn get_tr(&self) -> Box<QuadTree<T>>
    {
        self.tr.clone().unwrap()
    }

    pub fn get_bl(&self) -> Box<QuadTree<T>>
    {
        self.bl.clone().unwrap()
    }

    pub fn get_br(&self) -> Box<QuadTree<T>>
    {
        self.br.clone().unwrap()
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

    fn point_inside(&self, p : Vector) -> bool
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

    pub fn query(&mut self, p : Vector) -> &mut Vec<QuadValue<T>>
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

    pub fn query_no_mut(&mut self, p : Vector) -> &Vec<QuadValue<T>>
    {
        if self.is_leaf()
        {
            return &self.values
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
        &self.values
    }

    pub fn add(&mut self, v : T, p : Vector)
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
                self.tl.as_mut().unwrap().add(v.clone(), p);
            }
            if self.tr.as_mut().unwrap().point_inside(p)
            {
                self.tr.as_mut().unwrap().add(v.clone(), p);
            }
            if self.bl.as_mut().unwrap().point_inside(p)
            {
                self.bl.as_mut().unwrap().add(v.clone(), p);
            }
            if self.br.as_mut().unwrap().point_inside(p)
            {
                self.br.as_mut().unwrap().add(v.clone(), p);
            }
        }
    }

    pub fn change_pos(&mut self, p : Vector, p2 : Vector) -> Result<(), ()>
    {
        let o = self.remove(p);
        if o.is_none()
        {
            return Err(());
        }
        self.add(o.unwrap().1, p2);
        Ok(())
    }

    pub fn len(&self) -> usize
    {
        self.values.len()
    }

    pub fn remove(&mut self, p : Vector) -> Option<QuadValue<T>>
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

    pub fn find(&self, value : T) -> Option<Vector>
    {
        if self.is_leaf()
        {
            for i in 0..self.values.len()
            {
                if self.values[i].1 == value
                {
                    return Some(self.values[i].0);
                }
            }
        } else 
        {
            if let Some(v) = self.tl.as_ref().unwrap().find(value.clone())
            {
                return Some(v);
            }
            if let Some(v) = self.tr.as_ref().unwrap().find(value.clone())
            {
                return Some(v);
            }
            if let Some(v) = self.bl.as_ref().unwrap().find(value.clone())
            {
                return Some(v);
            }
            if let Some(v) = self.br.as_ref().unwrap().find(value.clone())
            {
                return Some(v);
            }
        }
        None
    }

    pub fn is_leaf(&self) -> bool
    {
        return self.tr.is_none()
    }
}