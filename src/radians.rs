use std::f64::consts::PI;

/// degrees to radians
pub fn to_radians(deg : f64) -> f64
{
    deg * (PI / 180.)
}

/// radians to degrees
pub fn to_degrees(rad : f64) -> f64
{
    rad * (180. / PI)
}