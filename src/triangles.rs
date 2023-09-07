use crate::{Vector, line::edge_function};

pub struct Triangle2D
{
    pub p1 : Vector,
    pub p2 : Vector,
    pub p3 : Vector
}

impl Triangle2D
{
    pub fn new(p1 : Vector, p2 : Vector, p3 : Vector) -> Self
    {
        Self
        {
            p1,
            p2,
            p3
        }
    }

    pub fn point_inside_triangle(&self, point : Vector) -> bool
    {
        let v0 = self.p1.into();
        let v1 = self.p2.into();
        let v2 = self.p3.into();
        edge_function((v0, v1), point) > 0.0 && edge_function((v1, v2), point) > 0.0 && edge_function((v2, v0), point) > 0.0
    }

    pub fn barycentric_coordinates(&self, point : Vector) -> Option<(f32, f32, f32)>
    {
        let v0 = self.p1.into();
        let v1 = self.p2.into();
        let v2 = self.p3.into();
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