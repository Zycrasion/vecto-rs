use std::f32::consts::PI;

/// degrees to radians
pub fn to_radians(deg : f32) -> f32
{
    deg * (PI / 180.)
}

/// radians to degrees
pub fn to_degrees(rad : f32) -> f32
{
    rad * (180. / PI)
}