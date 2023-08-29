use crate::{Vec2, line::edge_function};

pub fn point_inside_triangle<V : Into<Vec2>>(tri : (V, V, V), point : Vec2) -> bool
{
    let v0 = tri.0.into();
    let v1 = tri.1.into();
    let v2 = tri.2.into();
    edge_function((v0, v1), point) > 0.0 && edge_function((v1, v2), point) > 0.0 && edge_function((v2, v0), point) > 0.0
}

pub fn barycentric_coordinates<V : Into<Vec2>>(tri : (V, V, V), point : Vec2) -> Option<(f32, f32, f32)>
{
    let v0 = tri.0.into();
    let v1 = tri.1.into();
    let v2 = tri.2.into();
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