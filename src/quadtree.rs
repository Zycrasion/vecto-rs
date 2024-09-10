use std::time::{Duration, Instant};

use crate::linear::Vector;
use crate::spatial::AABB;
use crate::vec::VectorTrait;
use crate::Float;

#[derive(Debug)]
struct QuadBranch
{
    branches : Option<Box<(QuadBranch, QuadBranch, QuadBranch, QuadBranch)>>,
    contents : Vec<(usize, Vector)>
}

impl QuadBranch
{
    pub fn new() -> Self
    {
        Self
        {
            branches : None,
            contents : vec![]
        }
    }
}

/// QuadTree will only store indices
#[derive(Debug)]
pub struct QuadTree
{
    root : QuadBranch,
    half_size : Vector,
    midpoint : Vector,
    bb : AABB,
    pub min_size : Vector,
    pub max_items : usize
}

impl QuadTree
{
    pub fn new(bb : AABB) -> Self
    {
        Self
        {
            root: QuadBranch::new(),
            midpoint : bb.start + bb.size / 2.,
            half_size : bb.size / 2.,
            bb,
            min_size : bb.size / 10.,
            max_items : 100,
        }
    }

    fn get_new_branch(v : &Vector, current_midpoint : &mut Vector, current_size : &mut Vector, mut currently_peeked_branch : &mut QuadBranch)
    {
        if currently_peeked_branch.branches.is_none()
        {
            return;
        }

        let normalised = *v - *current_midpoint;
        match (normalised.x.is_sign_positive(), normalised.y.is_sign_positive())
        {
            (true, true) => {
                currently_peeked_branch = &mut currently_peeked_branch.branches.as_mut().unwrap().0;
            },
            (true, false) => {
                currently_peeked_branch = &mut currently_peeked_branch.branches.as_mut().unwrap().1;
            },
            (false, false) => {
                currently_peeked_branch = &mut currently_peeked_branch.branches.as_mut().unwrap().2;
            }
            (false, true) => {
                currently_peeked_branch = &mut currently_peeked_branch.branches.as_mut().unwrap().3;
            },
        }
        *current_size /= 2.;
        *current_midpoint += *current_size * Vector::new2(normalised.x.signum(), normalised.y.signum());
    }

    fn insert_en_bulk(p : Vec<(usize, Vector)>, peeked_branch : &mut QuadBranch, midpoint : Vector)
    {
        for v in p
        {
            let normalised = v.1 - midpoint;
            match (normalised.x.is_sign_positive(), normalised.y.is_sign_positive())
            {
                (true, true) => {
                    peeked_branch.branches.as_mut().unwrap().0.contents.push(v);
                },
                (true, false) => {
                    peeked_branch.branches.as_mut().unwrap().1.contents.push(v);
                },
                (false, false) => {
                    peeked_branch.branches.as_mut().unwrap().2.contents.push(v);
                }
                (false, true) => {
                    peeked_branch.branches.as_mut().unwrap().3.contents.push(v);
                },
            }
        }
    }

    pub fn query(&mut self, v : &Vector) -> Result<Vec<(usize, Vector)>, ()>
    {
        let mut normalised : Vector;
        let mut current_size = self.bb.size;
        let mut current_midpoint = self.midpoint;
        let mut currently_peeked_branch = &mut self.root;
        let instant = Instant::now();
        loop
        {
            if instant.elapsed() >= Duration::from_secs(5)
            {
                break;
            }
            if currently_peeked_branch.branches.is_none()
            {
                return Ok(currently_peeked_branch.contents.clone());
            }
        }

        Err(())
    }

    pub fn insert(&mut self, i : usize, v : &Vector) -> Result<(), ()>
    {
        if !self.bb.point_inside(*v)
        {
            return Err(())
        }

        let mut current_size = self.bb.size;
        let mut current_midpoint = self.midpoint;
        let mut currently_peeked_branch = &mut self.root;
        let instant = Instant::now();
        loop
        {
            if instant.elapsed() >= Duration::from_secs(5)
            {
                break;
            }
            if currently_peeked_branch.branches.is_none()
            {
                if currently_peeked_branch.contents.len() < self.max_items || current_size <= self.min_size
                {
                    currently_peeked_branch.contents.push((i, *v));
                    return Ok(())
                }

                // Subdivide the branch 
                let saved_contents = currently_peeked_branch.contents.drain(0..currently_peeked_branch.contents.len());

                currently_peeked_branch.branches = Some(Box::new((QuadBranch::new(),QuadBranch::new(),QuadBranch::new(),QuadBranch::new())));
                Self::insert_en_bulk(saved_contents.collect(), currently_peeked_branch, current_midpoint);
            }

            Self::get_new_branch(v, &mut current_midpoint, &mut current_size, currently_peeked_branch, current);
        }

        Err(())
    }
}