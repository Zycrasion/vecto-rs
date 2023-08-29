use crate::{Vec2, line::edge_function, Vec3};

pub fn point_inside_triangle<V : Into<Vec2>>(tri : (V, V, V), point : Vec2) -> bool
{
    let v0 = tri.0.into();
    let v1 = tri.1.into();
    let v2 = tri.2.into();
    edge_function((v0, v1), point) > 0.0 && edge_function((v1, v2), point) > 0.0 && edge_function((v2, v0), point) > 0.0
}
