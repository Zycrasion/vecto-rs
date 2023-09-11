use crate::{positional::Vector, line::edge_function};

/// Triangle Edge Descriptor
pub enum TriangleEdge
{
    AB,
    BC,
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
    pub fn get_edge(&self, edge : TriangleEdge) -> f32
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
        edge_function((v0, v1), point) > 0.0 && edge_function((v1, v2), point) > 0.0 && edge_function((v2, v0), point) > 0.0
    }

    /// Calculate barycentric coordinates
    pub fn barycentric_coordinates(&self, point : Vector) -> Option<(f32, f32, f32)>
    {
        let v0 = self.a.into();
        let v1 = self.b.into();
        let v2 = self.c.into();
        let area = edge_function((v0, v1), v2);
        let mut w0 = edge_function((v1, v2), point);
        let mut w1 = edge_function((v2, v0), point);
        let mut w2 = edge_function((v0, v1), point);

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