use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign}};

use num_traits::{Float, Num};

macro_rules! impl_number {
    ($($number_type:tt )+) => {
        $(impl BaseNumber for $number_type
        {
        })+
    };
}

pub trait BaseNumber : 
    Copy + 
    Clone + 
    AddAssign +
    SubAssign +
    MulAssign +
    DivAssign + 
    RemAssign +
    Debug +
    Num
{}

pub trait BaseFloat : 
    BaseNumber +
    Float
{}

impl_number!(i8 i16 i32 i64);
impl_number!(u8 u16 u32 u64);
impl_number!(f32 f64);