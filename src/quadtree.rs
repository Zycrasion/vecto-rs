use crate::linear::Vector;
use crate::spatial::AABB;
use crate::vec::VectorTrait;
use crate::Float;

struct QuadBranch
{
    branches : Option<Box<(QuadBranch, QuadBranch, QuadBranch, QuadBranch)>>,
    contents : Vec<usize>
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
pub struct QuadTree
{
    root : QuadBranch,
    half_size : Vector,
    midpoint : Vector,
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
        }
    }
}