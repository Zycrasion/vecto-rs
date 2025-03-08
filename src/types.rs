use core::f32;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

use num_traits::{Float, FloatConst, Num, NumCast};

macro_rules! impl_number {
    ($($number_type:tt )+) => {
        $(impl BaseNumber for $number_type
        {
        })+
    };
}

pub trait BaseNumber:
    Copy + Clone + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign + Debug + Num
{
}

pub trait BaseFloat: BaseNumber + Float + FloatConst + NumCast {}

impl_number!(i8 i16 i32 i64);
impl_number!(u8 u16 u32 u64);
impl_number!(f32 f64);

impl<T: Float + FloatConst + BaseNumber> BaseFloat for T {}
