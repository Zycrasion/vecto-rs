use crate::positional::Vector;

pub struct Line(pub Vector, pub Vector);

impl Line
{
    /// Determine if point is on a line or to the side
    /// 0 is on line
    /// -1 is on one side
    /// 1 is on the other
    pub fn edge_function(&self, point : Vector) -> f32
    {
        (point.x - self.0.x) * (self.1.y - self.0.y) - (point.y - self.0.y) * (self.1.x - self.0.x)
    }
}