use std::{
    fmt::{Debug, Display},
    ops::{self, *},
};
use crate::Float;

/// The Super Trait for all Vector types
pub trait VectorTrait:
    Sized
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Default
    + Debug
    + Display
    + Add<Self>
    + Sub<Self>
    + Mul<Self>
    + Div<Self>
    + Add<Float>
    + Sub<Float>
    + Mul<Float>
    + Div<Float>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + AddAssign<Float>
    + SubAssign<Float>
    + MulAssign<Float>
    + DivAssign<Float>
{
    /// Create a new vector using 2 paremeters
    fn new2(x: Float, y: Float) -> Self;

    /// Create a new vector using 3 paremeters
    fn new3(x: Float, y: Float, z: Float) -> Self;
    
    /// Get magnitude of vector
    fn magnitude(&self) -> Float;

    /// Clamp Vector between 2 Vector Types
    fn clamp(&self, min: Self, max: Self) -> Self;
    
    /// Get distance between 2 Vectors
    fn dist(&self, other: &Self) -> Float;
    
    /// Get Normalized Vector
    fn normalized(&self) -> Self;

    /// Get All fields and add them together (x+y+z)
    fn sum(&self) -> Float;
}

/// Vector Shorthand
pub fn vector3(x: Float, y: Float, z: Float) -> Vector {
    Vector::new3(x, y, z)
}

/// Vector
#[derive(Clone, Copy)]
pub struct Vector {
    /// X Coordinate of Vector
    pub x: Float,

    /// Y Coordinate of Vector
    pub y: Float,

    /// Z Coordinate of Vector
    pub z: Float,
}

impl Vector
{
    /// Cross product
    pub fn cross(a : &Vector, b : &Vector) -> Vector
    {
        Vector::new3(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }

    /// Rotate around the x axis
    pub fn rotate_x(&mut self, angle : Float)
    {
        let cos = angle.cos();
        let sin = angle.sin();
        let y = self.y;
        let z = self.z;
        
        self.y = y * cos - z * sin;
        self.z = y * sin + z * cos;
    }

    /// Rotate around the y axis
    pub fn rotate_y(&mut self, angle : Float)
    {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x;
        let z = self.z;
        
        self.x =  x * cos + z * sin;
        self.z = -x * sin + z * cos;
    }
    
    /// Rotate around the z axis
    pub fn rotate_z(&mut self, angle : Float)
    {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x;
        let y = self.y;
        
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
    }
}

impl VectorTrait for Vector {
    /// Create a Vector2
    fn new2(x: Float, y: Float) -> Self {
        Self { x, y, z: 0.0 }
    }

    /// Create a Vector3
    fn new3(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    /// Get Length of Vector
    fn magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Clamp Vector between points
    fn clamp(&self, min: Self, max: Self) -> Self {
        let mut new_vec = *self;
        new_vec.x = new_vec.x.clamp(min.x, max.x);
        new_vec.y = new_vec.y.clamp(min.y, max.y);
        new_vec.z = new_vec.z.clamp(min.z, max.z);
        new_vec
    }

    /// Distance Between 2 Vectors
    fn dist(&self, other: &Self) -> Float {
        (*other - *self).magnitude().abs()
    }

    /// Get Normalised Vector
    fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    /// Get Sum (all parts added together)
    fn sum(&self) -> Float {
        self.x + self.y + self.z
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.x.partial_cmp(&other.x);
        let b = self.y.partial_cmp(&other.y);
        let c = self.z.partial_cmp(&other.z);

        if a == b && b == c {
            a
        } else {
            None
        }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x : {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

vec_vec_op!(Add, add, +);
vec_vec_op!(Sub, sub, -);
vec_vec_op!(Mul, mul, *);
vec_vec_op!(Div, div, /);

vec_vec_assign_op!(AddAssign, add_assign, +=);
vec_vec_assign_op!(SubAssign, sub_assign, -=);
vec_vec_assign_op!(MulAssign, mul_assign, *=);
vec_vec_assign_op!(DivAssign, div_assign, /=);

vec_Float_op!(Add, add, +);
vec_Float_op!(Sub, sub, -);
vec_Float_op!(Mul, mul, *);
vec_Float_op!(Div, div, /);

vec_Float_assign_op!(AddAssign, add_assign, +=);
vec_Float_assign_op!(SubAssign, sub_assign, -=);
vec_Float_assign_op!(MulAssign, mul_assign, *=);
vec_Float_assign_op!(DivAssign, div_assign, /=);

impl From<(Float, Float)> for Vector {
    fn from(value: (Float, Float)) -> Self {
        Self::new2(value.0, value.1)
    }
}

impl Into<(Float, Float)> for Vector {
    fn into(self) -> (Float, Float) {
        (self.x, self.y)
    }
}

impl From<(Float, Float, Float)> for Vector {
    fn from(value: (Float, Float, Float)) -> Self {
        Self::new3(value.0, value.1, value.2)
    }
}

impl Into<(Float, Float, Float)> for Vector {
    fn into(self) -> (Float, Float, Float) {
        (self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone)]
/// Vector 4
pub struct Vector4 {
    /// X Coordinate of Vector
    pub x: Float,

    /// Y Coordinate of Vector
    pub y: Float,

    /// Z Coordinate of Vector
    pub z: Float,

    /// W Coordinate of Vector
    pub w: Float,
}

impl Vector4 {
    /// Create a new Vector4
    pub fn new4(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self { x, y, z, w }
    }
}

impl VectorTrait for Vector4 {
    /// Create a Vector2
    fn new2(x: Float, y: Float) -> Self {
        Self {
            x,
            y,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a Vector3
    fn new3(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Get Length of Vector
    fn magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    /// Clamp Vector between points
    fn clamp(&self, min: Self, max: Self) -> Self {
        let mut new_vec = *self;
        new_vec.x = new_vec.x.clamp(min.x, max.x);
        new_vec.y = new_vec.y.clamp(min.y, max.y);
        new_vec.z = new_vec.z.clamp(min.z, max.z);
        new_vec.w = new_vec.w.clamp(min.w, max.w);
        new_vec
    }

    /// Distance Between 2 Vectors
    fn dist(&self, other: &Self) -> Float {
        (*other - *self).magnitude().abs()
    }

    /// Get Normalised Vector
    fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    /// Get Sum (all parts added together)
    fn sum(&self) -> Float {
        self.x + self.y + self.z + self.w
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl PartialOrd for Vector4 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.x.partial_cmp(&other.x);
        let b = self.y.partial_cmp(&other.y);
        let c = self.z.partial_cmp(&other.z);
        let d = self.w.partial_cmp(&other.w);

        if a == b && b == c && c == d {
            a
        } else {
            None
        }
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}

impl Debug for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x : {}, y: {}, z: {}, w: {})",
            self.x, self.y, self.z, self.w
        )
    }
}

vec4_vec_op!(Add, add, +);
vec4_vec_op!(Sub, sub, -);
vec4_vec_op!(Mul, mul, *);
vec4_vec_op!(Div, div, /);

vec4_vec_assign_op!(AddAssign, add_assign, +=);
vec4_vec_assign_op!(SubAssign, sub_assign, -=);
vec4_vec_assign_op!(MulAssign, mul_assign, *=);
vec4_vec_assign_op!(DivAssign, div_assign, /=);

vec4_Float_op!(Add, add, +);
vec4_Float_op!(Sub, sub, -);
vec4_Float_op!(Mul, mul, *);
vec4_Float_op!(Div, div, /);

vec4_Float_assign_op!(AddAssign, add_assign, +=);
vec4_Float_assign_op!(SubAssign, sub_assign, -=);
vec4_Float_assign_op!(MulAssign, mul_assign, *=);
vec4_Float_assign_op!(DivAssign, div_assign, /=);