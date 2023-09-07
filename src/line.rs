use crate::Vector;

pub fn edge_function(line : (Vector, Vector), point : Vector) -> f32
{
    (point.x - line.0.x) * (line.1.y - line.0.y) - (point.y - line.0.y) * (line.1.x - line.0.x)
}