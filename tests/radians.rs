use std::f32::consts::PI;
pub use vecto_rs::prelude::*;

#[test]
pub fn radians_to_degrees()
{
    assert_eq!(to_degrees(PI / 2.), 90.);
    assert_eq!(to_degrees(PI), 180.);
    assert_eq!(to_degrees(2. * PI), 360.);
}

#[test]
pub fn degrees_to_radians()
{
    assert_eq!(to_radians(90.), PI / 2.);
    assert_eq!(to_radians(180.), PI);
    assert_eq!(to_radians(360.),  PI * 2.);
}