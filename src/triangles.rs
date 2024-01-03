use crate::{linear::Vector, trig::Line, vec::VectorTrait};
use crate::Float;

/// Triangle Edge Descriptor
pub enum TriangleEdge
{
    /// Edge from Points A to B
    AB,

    /// Edge from Points B to C
    BC,

    /// Edge from Points C to A
    CA
}

/// Triangle class
pub struct Triangle
{
    /// Position of point a
    pub a : Vector,

    /// Position of point b
    pub b : Vector,

    /// Position of point c
    pub c : Vector
}

impl Triangle
{
    /// Create new Triangle
    pub fn new(a : Vector, b : Vector, c : Vector) -> Self
    {
        Self
        {
            a,
            b,
            c
        }
    }

    /// Get length of edge
    pub fn get_edge(&self, edge : TriangleEdge) -> Float
    {
        match edge
        {
            TriangleEdge::AB => self.a.dist(&self.b),
            TriangleEdge::BC => self.b.dist(&self.c),
            TriangleEdge::CA => self.c.dist(&self.a),
        }
    }

    /// Check if point inside triangle
    pub fn point_inside_triangle(&self, point : Vector) -> bool
    {
        let v0 = self.a.into();
        let v1 = self.b.into();
        let v2 = self.c.into();
        let a  = Line(v0, v1);
        let b = Line(v1, v2);
        let c = Line(v2, v0);
        a.edge_function(point) > 0.0 && b.edge_function(point) > 0.0 && c.edge_function(point) > 0.0
    }

    /// Calculate barycentric coordinates
    pub fn barycentric_coordinates(&self, point : Vector) -> Option<(Float, Float, Float)>
    {
        let v0 = self.a.into();
        let v1 = self.b.into();
        let v2 = self.c.into();
        let area =   Line(v0, v1).edge_function(v2);
        let mut w0 = Line(v1, v2).edge_function(point);
        let mut w1 = Line(v2, v0).edge_function(point);
        let mut w2 = Line(v0, v1).edge_function(point);

        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
        {
            w0 /= area;
            w1 /= area;
            w2 /= area;
            Some((w0, w1, w2))
        } else {
            None
        }
    }
}