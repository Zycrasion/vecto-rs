/// The float precision that the library was compiled with
#[cfg(not(feature = "f64"))]
pub type Float = f32;

/// The float precision that the library was compiled with
#[cfg(feature = "f64")]
pub type Float = f64;

/// PI using the precision that the library was compiled with
#[cfg(not(feature = "f64"))]
pub const PI : Float = std::f32::consts::PI;

/// PI using the precision that the library was compiled with
#[cfg(feature = "f64")]
pub const PI : Float = std::f64::consts::PI;