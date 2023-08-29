use crate::Vec2;

pub fn edge_function(line : (Vec2, Vec2), point : Vec2) -> f32
{
    (point.x() - line.0.x()) * (line.1.y() - line.0.y()) - (point.y() - line.0.y()) * (line.1.x() - line.0.x())
}