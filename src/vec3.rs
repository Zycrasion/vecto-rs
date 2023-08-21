use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3
{
    pub const UP : Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const ONE : Vec3 = Vec3(1.0, 1.0, 1.0);
    pub const ZERO : Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const RIGHT : Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const FORWARD : Vec3 = Vec3(0.0, 0.0, 1.0);

    pub fn magnitude(&self) -> f32
    {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn dist(&self, rhs : &Vec3) -> f32
    {
        (self.clone() - rhs.clone()).magnitude().abs()
    }

    pub fn clamp(&self, min : Vec3, max : Vec3) -> Self
    {
        let mut new_vec = self.clone();
        new_vec.0 = new_vec.0.clamp(min.0, max.0);
        new_vec.1 = new_vec.1.clamp(min.1, max.1);
        new_vec.2 = new_vec.2.clamp(min.2, max.2);
        new_vec
    }

    pub fn normalized(&self) -> Self
    {
        self.clone() / self.magnitude()
    }

    pub fn x<T : TryFrom<f32> >(&self) -> T where <T as TryFrom<f32>>::Error: Debug
    {
        self.0.try_into().unwrap()
    }

    pub fn y<T : TryFrom<f32> >(&self) -> T where <T as TryFrom<f32>>::Error: Debug
    {
        self.1.try_into().unwrap()
    }

    pub fn z<T : TryFrom<f32> >(&self) -> T where <T as TryFrom<f32>>::Error: Debug
    {
        self.2.try_into().unwrap()
    }
}

impl PartialEq for Vec3
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialOrd for Vec3
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0.partial_cmp(&other.0) == self.1.partial_cmp(&other.1) && self.0.partial_cmp(&other.0) == self.2.partial_cmp(&other.2)
        {
            return Some(self.0.partial_cmp(&other.0).unwrap());
        }
        None
    }
}

impl Default for Vec3
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl Debug for Vec3
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec3").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}

impl Display for Vec3
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.0, self.1, self.2)
    }
}

impl std::ops::Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Sub<f32> for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl std::ops::Add for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Add<f32> for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 - rhs)
    }
}

impl std::ops::Mul for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Mul<f32> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div for Vec3
{
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl std::ops::Div<f32> for Vec3
{
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl From<(f32, f32, f32)> for Vec3
{
    fn from(value: (f32, f32, f32)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Into<(f32, f32, f32)> for Vec3
{
    fn into(self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}