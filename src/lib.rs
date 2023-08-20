use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2(pub f32, pub f32);

impl Vec2
{
    pub const ZERO : Vec2 = Vec2(0.0, 0.0);
    pub const ONE : Vec2 = Vec2(1.0, 1.0);
    pub const RIGHT : Vec2 = Vec2(1.0, 0.0);
    pub const UP : Vec2 = Vec2(0.0, 1.0);

    pub fn magnitude(&self) -> f32
    {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn dist(&self, rhs : &Vec2) -> f32
    {
        (self.clone() - rhs.clone()).magnitude().abs()
    }

    pub fn clamp(&self, min : Vec2, max : Vec2) -> Self
    {
        let mut new_vec = self.clone();
        new_vec.0 = new_vec.0.clamp(min.0, max.0);
        new_vec.1 = new_vec.1.clamp(min.1, max.1);
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
}

impl Debug for Vec2
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec2").field(&self.0).field(&self.1).finish()
    }
}

impl Display for Vec2
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.0, self.1)
    }
}

impl std::ops::Sub for Vec2
{
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Sub<f32> for Vec2
{
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec2(self.0 - rhs, self.1 - rhs)
    }
}

impl std::ops::Add for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<f32> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        Vec2(self.0 + rhs, self.1 + rhs)
    }
}

impl std::ops::Mul for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::Mul<f32> for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Div for Vec2
{
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl std::ops::Div<f32> for Vec2
{
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl From<(f32, f32)> for Vec2
{
    fn from(value: (f32, f32)) -> Self {
        Self(value.0, value.1)
    }
}

impl Into<(f32, f32)> for Vec2
{
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}