use crate::positional::Vector;

/// Determine if point is on a line or to the side
/// 0 is on line
/// -1 is on one side
/// 1 is on the other
pub fn edge_function(line : (Vector, Vector), point : Vector) -> f32
{
    (point.x - line.0.x) * (line.1.y - line.0.y) - (point.y - line.0.y) * (line.1.x - line.0.x)
}