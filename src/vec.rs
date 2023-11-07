use std::{fmt::{Debug, Display}, ops};

pub struct Vector3(pub f32, pub f32, pub f32);
impl Into<Vector> for Vector3
{
    fn into(self) -> Vector {
        Vector { x: self.0, y: self.1, z: self.2 }
    }
}

/// Vector
#[derive(Clone, Copy)]
pub struct Vector
{
    /// X Coordinate of Vector
    pub x : f32,

    /// Y Coordinate of Vector
    pub y : f32,

    /// Z Coordinate of Vector
    pub z : f32,
}

impl Vector
{
    /// Create a Vector2
    pub fn new2(x : f32 , y : f32) -> Self
    {
        Self { x, y, z: 0.0 }
    }

    /// Create a Vector3
    pub fn new3(x : f32 , y : f32, z : f32) -> Self
    {
        Self { x, y, z }
    }

    /// Get Length of Vector
    pub fn magnitude(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Distance Between 2 Vectors
    pub fn dist(&self, other: &Self) -> f32
    {
        (*other - *self).magnitude().abs()
    }

    /// Clamp Vector between points
    pub fn clamp(&self, min : Self, max : Self) -> Self
    {
        let mut new_vec = *self;
        new_vec.x = new_vec.x.clamp(min.x, max.x);
        new_vec.y = new_vec.y.clamp(min.y, max.y);
        new_vec.z = new_vec.z.clamp(min.z, max.z);
        new_vec
    }

    /// Get Normalised Vector
    pub fn normalized(&self) -> Self
    {
        *self / self.magnitude()
    }
}

impl PartialEq for Vector
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Vector
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.x.partial_cmp(&other.x);
        let b = self.y.partial_cmp(&other.y);
        let c = self.z.partial_cmp(&other.z);

        if a == b && b == c
        {
            a
        } else
        {
            None
        }
    }
}

impl Default for Vector
{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}

impl Debug for Vector
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}

impl Display for Vector
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x : {}, y: {}, z: {}", self.x, self.y, self.z)
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

vec_f32_op!(Add, add, +);
vec_f32_op!(Sub, sub, -);
vec_f32_op!(Mul, mul, *);
vec_f32_op!(Div, div, /);

vec_f32_assign_op!(AddAssign, add_assign, +=);
vec_f32_assign_op!(SubAssign, sub_assign, -=);
vec_f32_assign_op!(MulAssign, mul_assign, *=);
vec_f32_assign_op!(DivAssign, div_assign, /=); 

impl From<(f32, f32)> for Vector
{
    fn from(value: (f32, f32)) -> Self {
        Self::new2(value.0, value.1)
    }
}

impl Into<(f32, f32)> for Vector
{
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl From<(f32, f32, f32)> for Vector
{
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new3(value.0, value.1, value.2)
    }
}

impl Into<(f32, f32, f32)> for Vector
{
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}