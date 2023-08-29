use crate::{Vec2, line::edge_function};

pub fn point_inside_triangle(tri : (Vec2, Vec2, Vec2), point : Vec2) -> bool
{
    edge_function((tri.0, tri.1), point) > 0.0 && edge_function((tri.1, tri.2), point) > 0.0 && edge_function((tri.2, tri.0), point) > 0.0
}