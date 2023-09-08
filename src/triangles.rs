use crate::{Vector, line::edge_function};

pub enum TriangleEdge
{
    AB,
    BC,
    CA
}

pub struct Triangle
{
    pub a : Vector,
    pub b : Vector,
    pub c : Vector
}

impl Triangle
{
    pub fn new(a : Vector, b : Vector, c : Vector) -> Self
    {
        Self
        {
            a,
            b,
            c
        }
    }

    pub fn get_edge(&self, edge : TriangleEdge) -> f32
    {
        match edge
        {
            TriangleEdge::AB => self.a.dist(&self.b),
            TriangleEdge::BC => self.b.dist(&self.a),
            TriangleEdge::CA => self.c.dist(&self.a),
        }
    }

    pub fn point_inside_triangle(&self, point : Vector) -> bool
    {
        let v0 = self.a.into();
        let v1 = self.b.into();
        let v2 = self.c.into();
        edge_function((v0, v1), point) > 0.0 && edge_function((v1, v2), point) > 0.0 && edge_function((v2, v0), point) > 0.0
    }

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