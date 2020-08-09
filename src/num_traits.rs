use std::convert::{TryFrom, TryInto};

pub trait Number: Clone + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait RealNumber: Number {}

pub trait RealType: RealNumber + Into<f64> {}
pub trait IntegerType: RealNumber + TryFrom<isize> + TryInto<isize> {}

macro_rules! impl_number_trait {
    ($($t:ty),+) => (
        $(impl Number for $t {
            fn zero() -> Self { 0 as $t }
            fn one() -> Self { 1 as $t }
        })*
    )
}

macro_rules! impl_real_number {
    ($($t:ty),+) => ($(impl RealNumber for $t {})*)
}

macro_rules! impl_real_type {
    ($($t:ty),+) => ($(impl RealType for $t {})*)
}

macro_rules! impl_int_type {
    ($($t:ty),+) => ($(impl IntegerType for $t {})*)
}

impl_number_trait!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
impl_real_number!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
impl_real_type!(f32, f64);
impl_int_type!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
