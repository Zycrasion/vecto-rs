use crate::{Float, PI};

/// degrees to radians
pub fn to_radians(deg : Float) -> Float
{
    deg * (PI / 180.)
}

/// radians to degrees
pub fn to_degrees(rad : Float) -> Float
{
    rad * (180. / PI)
}